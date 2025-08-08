import sys
import os
import argparse
import numpy as np
import pyvista as pv


def plot_2d_streamlines(vtk_file, density_threshold=0.01, show_solid=True, theme='light'):
    """
    Plot 2D streamlines from a VTK file (STRUCTURED_POINTS) using PyVista.
    """
    grid = pv.read(vtk_file)
    if 'velocity' not in grid.array_names:
        raise ValueError('No velocity field found in VTK file.')

    # Mask solid if present
    if show_solid and 'solid' in grid.array_names:
        solid = grid['solid']
        mask = solid < 1  # 0=fluid, 1=solid
        grid = grid.extract_points(mask)
    elif 'density' in grid.array_names:
        mask = grid['density'] > density_threshold
        grid = grid.extract_points(mask)

    # 2D: Take a slice at the middle z-plane
    bounds = grid.bounds
    z_mid = 0.5 * (bounds[4] + bounds[5])
    slice_ = grid.slice(normal='z', origin=(0, 0, z_mid))

    # Streamlines
    seed = slice_.center  # or use a grid/array of points for more streamlines
    stream = slice_.streamlines_from_source(
        seed,
        vectors='velocity',
        n_points=200,
        max_time=200.0,
        initial_step_length=1.0,
        terminal_speed=1e-5,
        integrator_type=45,
    )

    # Compute velocity magnitude for coloring
    if 'velocity' in slice_.array_names:
        vel = slice_['velocity']
        vel_mag = np.linalg.norm(vel, axis=1)
        stream['vel_mag'] = vel_mag[:stream.n_points]
        stream_scalars = 'vel_mag'
        vmin, vmax = np.nanmin(vel_mag), np.nanmax(vel_mag)
    else:
        stream_scalars = None
        vmin, vmax = None, None

    # Theme selection
    theme_opts = {
        'light':  {'bg': 'white', 'density_cmap': 'turbo', 'stream_cmap': 'plasma'},
        'dark':   {'bg': 'black', 'density_cmap': 'viridis', 'stream_cmap': 'cool'},
        'paraview': {'bg': '#52576e', 'density_cmap': 'coolwarm', 'stream_cmap': 'hot'},
        'matplotlib': {'bg': 'white', 'density_cmap': 'viridis', 'stream_cmap': 'inferno'},
        'seaborn': {'bg': '#f0f0f0', 'density_cmap': 'crest', 'stream_cmap': 'rocket'},
    }
    t = theme_opts.get(theme, theme_opts['light'])
    p = pv.Plotter()
    p.set_background(t['bg'])
    # Normalize density colormap
    if 'density' in slice_.array_names:
        dens = slice_['density']
        dmin, dmax = np.nanmin(dens), np.nanmax(dens)
        p.add_mesh(slice_, scalars='density', cmap=t['density_cmap'], opacity=0.5, clim=(dmin, dmax))
    else:
        p.add_mesh(slice_, opacity=0.5)
    # Normalize velocity colormap
    p.add_mesh(
        stream.tube(radius=0.2),
        scalars=stream_scalars,
        cmap=t['stream_cmap'],
        clim=(vmin, vmax) if vmin is not None and vmax is not None else None,
        show_scalar_bar=True,
        scalar_bar_args={'title': 'Velocity'},
    )
    p.show(title=f'2D Streamlines ({theme} theme)')


def plot_3d_streamlines(vtk_file, density_threshold=0.01, show_solid=True, theme='light'):
    """
    Plot 3D streamlines from a VTK file (STRUCTURED_POINTS) using PyVista.
    """
    grid = pv.read(vtk_file)
    if 'velocity' not in grid.array_names:
        raise ValueError('No velocity field found in VTK file.')

    # Mask solid if present
    if show_solid and 'solid' in grid.array_names:
        solid = grid['solid']
        mask = solid < 1  # 0=fluid, 1=solid
        grid = grid.extract_points(mask)
    elif 'density' in grid.array_names:
        mask = grid['density'] > density_threshold
        grid = grid.extract_points(mask)

    # Seed points: grid or random
    bounds = grid.bounds
    x = np.linspace(bounds[0], bounds[1], 10)
    y = np.linspace(bounds[2], bounds[3], 10)
    z = np.linspace(bounds[4], bounds[5], 10)
    seed = pv.PolyData(np.array(np.meshgrid(x, y, z)).reshape(3, -1).T)

    stream = grid.streamlines_from_source(
        seed,
        vectors='velocity',
        max_time=200.0,
        initial_step_length=1.0,
        terminal_speed=1e-5,
        integrator_type=45,
    )

    # Compute velocity magnitude for coloring
    if 'velocity' in grid.array_names:
        vel = grid['velocity']
        vel_mag = np.linalg.norm(vel, axis=1)
        stream['vel_mag'] = vel_mag[:stream.n_points]
        stream_scalars = 'vel_mag'
        vmin, vmax = np.nanmin(vel_mag), np.nanmax(vel_mag)
    else:
        stream_scalars = None
        vmin, vmax = None, None

    # Theme selection
    theme_opts = {
        'light':  {'bg': 'white', 'density_cmap': 'turbo', 'stream_cmap': 'plasma'},
        'dark':   {'bg': 'black', 'density_cmap': 'viridis', 'stream_cmap': 'cool'},
        'paraview': {'bg': '#52576e', 'density_cmap': 'coolwarm', 'stream_cmap': 'hot'},
        'matplotlib': {'bg': 'white', 'density_cmap': 'viridis', 'stream_cmap': 'inferno'},
        'seaborn': {'bg': '#f0f0f0', 'density_cmap': 'crest', 'stream_cmap': 'rocket'},
    }
    t = theme_opts.get(theme, theme_opts['light'])
    p = pv.Plotter()
    p.set_background(t['bg'])
    # Normalize density colormap
    if 'density' in grid.array_names:
        dens = grid['density']
        dmin, dmax = np.nanmin(dens), np.nanmax(dens)
        p.add_mesh(grid, scalars='density', cmap=t['density_cmap'], opacity=0.3, clim=(dmin, dmax))
    else:
        p.add_mesh(grid, opacity=0.3)
    # Normalize velocity colormap
    p.add_mesh(
        stream.tube(radius=0.2),
        scalars=stream_scalars,
        cmap=t['stream_cmap'],
        clim=(vmin, vmax) if vmin is not None and vmax is not None else None,
        show_scalar_bar=True,
        scalar_bar_args={'title': 'Velocity'},
    )
    p.show(title=f'3D Streamlines ({theme} theme)')


def main():
    parser = argparse.ArgumentParser(description='Plot 2D/3D fluid streamlines from VTK.')
    parser.add_argument('vtk_file', help='Path to VTK file (STRUCTURED_POINTS)')
    parser.add_argument('-d', choices=['2', '3'], required=True, help='Plot 2D or 3D streamlines')
    parser.add_argument('-t', '--theme', choices=['light', 'dark', 'paraview', 'matplotlib', 'seaborn'], default='light', help='Color theme for plot')
    parser.add_argument('--no-solid', action='store_true', help='Ignore solid mask if present')
    args = parser.parse_args()

    if args.d == '2':
        plot_2d_streamlines(args.vtk_file, show_solid=not args.no_solid, theme=args.theme)
    else:
        plot_3d_streamlines(args.vtk_file, show_solid=not args.no_solid, theme=args.theme)


if __name__ == '__main__':
    main()
