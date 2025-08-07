import sys
import numpy as np
import pyvista as pv
from pyvistaqt import QtInteractor
from PyQt5 import QtWidgets, QtCore

class VTKViewer(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('CappuSim VTK Viewer')
        self.resize(1200, 800)

        # Barra de menu superior
        menubar = self.menuBar()
        file_menu = menubar.addMenu('Arquivo')
        open_action = QtWidgets.QAction('Abrir VTK', self)
        open_action.triggered.connect(self.open_file)
        file_menu.addAction(open_action)
        exit_action = QtWidgets.QAction('Sair', self)
        exit_action.triggered.connect(self.close)
        file_menu.addAction(exit_action)
        view_menu = menubar.addMenu('Visualização')
        help_menu = menubar.addMenu('Ajuda')

        # Widget central com layout horizontal
        self.central_widget = QtWidgets.QWidget()
        self.setCentralWidget(self.central_widget)
        main_layout = QtWidgets.QHBoxLayout(self.central_widget)

        # Menu lateral esquerdo
        sidebar = QtWidgets.QFrame()
        sidebar.setFixedWidth(220)
        sidebar_layout = QtWidgets.QVBoxLayout(sidebar)
        sidebar_layout.setAlignment(QtCore.Qt.AlignTop)

        # Botão abrir arquivo
        self.btn_open = QtWidgets.QPushButton('Abrir VTK')
        sidebar_layout.addWidget(self.btn_open)
        self.btn_open.clicked.connect(self.open_file)

        # Tema
        theme_row = QtWidgets.QHBoxLayout()
        theme_row.addWidget(QtWidgets.QLabel('Tema:'))
        self.theme_box = QtWidgets.QComboBox()
        self.theme_box.addItems(['light', 'dark'])
        theme_row.addWidget(self.theme_box)
        sidebar_layout.addLayout(theme_row)
        self.theme_box.currentTextChanged.connect(self.update_theme)
        self.theme = 'light'

        # Mostrar sólidos
        self.chk_solids = QtWidgets.QCheckBox('Mostrar sólidos')
        self.chk_solids.setChecked(True)
        sidebar_layout.addWidget(self.chk_solids)
        self.chk_solids.stateChanged.connect(self.update_solids)
        self.show_solids = True

        # Barra de progresso
        self.progress = QtWidgets.QProgressBar()
        self.progress.setMinimum(0)
        self.progress.setMaximum(0)
        self.progress.setVisible(False)
        sidebar_layout.addWidget(self.progress)

        # Seleção de campo
        scalar_field_row = QtWidgets.QHBoxLayout()
        scalar_field_row.addWidget(QtWidgets.QLabel('Campo escalar:'))
        self.scalar_field_box = QtWidgets.QComboBox()
        self.scalar_field_box.addItems(['Nenhum', 'density', 'velocity', 'vorticity', 'q_criterion'])
        self.scalar_field_box.setCurrentText('Nenhum')
        scalar_field_row.addWidget(self.scalar_field_box)
        sidebar_layout.addLayout(scalar_field_row)

        # Streamlines
        stream_field_row = QtWidgets.QHBoxLayout()
        stream_field_row.addWidget(QtWidgets.QLabel('Streamlines:'))
        self.stream_field_box = QtWidgets.QComboBox()
        self.stream_field_box.addItems(['Nenhum', 'velocity', 'vorticity'])
        stream_field_row.addWidget(self.stream_field_box)
        sidebar_layout.addLayout(stream_field_row)

        sidebar_layout.addStretch(1)

        # Adiciona sidebar ao layout principal
        main_layout.addWidget(sidebar)

        # PyVistaQt plotter ocupa o restante
        self.plotter = QtInteractor(self.central_widget)
        main_layout.addWidget(self.plotter.interactor, stretch=1)

        # Estado
        self.grid = None
        self.filename = None
        self.current_field = 'Nenhum'

        # Conexões
        self.scalar_field_box.currentTextChanged.connect(self.on_scalar_field_changed)
        self.stream_field_box.currentTextChanged.connect(self.on_stream_field_changed)
        
        # Aplicar estilo inicial
        self.update_ui_style('Default')

    def update_theme(self, theme):
        self.theme = theme
        
        # Aplica estilo da interface baseado no tema
        if theme == 'dark':
            self.update_ui_style('Dark')
        else:  # light ou qualquer outro valor padrão
            self.update_ui_style('Default')
            
        if self.grid is not None:
            # Sempre atualiza a visualização quando o tema muda
            stream_field = self.stream_field_box.currentText()
            if stream_field != 'Nenhum':
                self.show_streamlines(stream_field)
            else:
                self.show_field(self.current_field)

    def update_solids(self, state):
        self.show_solids = bool(state)
        print(f"Debug: update_solids chamado com state={state}, show_solids={self.show_solids}")
        
        if self.grid is not None:
            # Remove qualquer geometria de sólidos existente
            try:
                self.plotter.remove_actor('solids')
            except ValueError:
                pass 
                
            # Sempre atualiza a visualização quando a opção de sólidos muda
            stream_field = self.stream_field_box.currentText()
            if stream_field != 'Nenhum':
                self.show_streamlines(stream_field)
            else:
                self.show_field(self.current_field)

    def add_scalar_field_to_plot(self, grid, field, theme_opts):
        """Adiciona um campo escalar ao plot atual"""
        if field == 'velocity' and 'velocity' in grid.array_names:
            vel = grid['velocity']
            vel_mag = np.linalg.norm(vel, axis=1)
            vmin, vmax = np.nanmin(vel_mag), np.nanmax(vel_mag)
            self.plotter.add_mesh(grid, scalars=vel_mag, cmap=theme_opts['vel_cmap'], opacity=0.3, clim=(vmin, vmax), show_scalar_bar=False)
        elif field == 'vorticity' and 'vorticity' in grid.array_names:
            vort = grid['vorticity']
            vort_mag = np.linalg.norm(vort, axis=1)
            vmin, vmax = np.nanmin(vort_mag), np.nanmax(vort_mag)
            self.plotter.add_mesh(grid, scalars=vort_mag, cmap=theme_opts['vort_cmap'], opacity=0.3, clim=(vmin, vmax), show_scalar_bar=False)
        elif field == 'q_criterion' and 'q_criterion' in grid.array_names:
            try:
                surf = grid.contour(isosurfaces=[0.0], scalars='q_criterion')
                self.plotter.add_mesh(surf, color='red', opacity=0.3, show_scalar_bar=False)
            except Exception:
                pass
        elif field == 'density' and 'density' in grid.array_names:
            dens = grid['density']
            dmin, dmax = np.nanmin(dens), np.nanmax(dens)
            self.plotter.add_mesh(grid, scalars='density', cmap=theme_opts['density_cmap'], opacity=0.3, clim=(dmin, dmax), show_scalar_bar=False)

    def add_solids_to_plot(self):
        """Adiciona geometria dos sólidos ao plot se a opção estiver habilitada"""
        if self.show_solids and self.grid and 'solid' in self.grid.array_names:
            solid_data = self.grid['solid']
            solid_mask = solid_data >= 1
            print(f"Debug: solid_data min={np.min(solid_data)}, max={np.max(solid_data)}, count_solids={np.sum(solid_mask)}")
            
            if solid_mask.any():
                solid_grid = self.grid.extract_points(solid_mask)
                self.plotter.add_mesh(
                    solid_grid, 
                    color='darkgray', 
                    opacity=0.8, 
                    show_scalar_bar=False,
                    name='solids'
                )
                print("Debug: Sólidos adicionados ao plot")

    def on_scalar_field_changed(self, field):
        """Gerencia mudanças no campo escalar, considerando se streamlines estão ativas"""
        stream_field = self.stream_field_box.currentText()
        if stream_field != 'Nenhum':
            # Se há streamlines ativas, atualiza a visualização combinada
            self.show_streamlines(stream_field)
        else:
            # Se não há streamlines, mostra apenas o campo escalar
            self.show_field(field)

    def on_stream_field_changed(self, field):
        """Gerencia mudanças no campo de streamlines"""
        if field == 'Nenhum':
            # Se "Nenhum" foi selecionado para streamlines, mostra apenas o campo escalar
            scalar_field = self.scalar_field_box.currentText()
            self.show_field(scalar_field)
        else:
            self.show_streamlines(field)
    def show_streamlines(self, field):
        if self.grid is None:
            return
        
        # Se "Nenhum" foi selecionado, não faz nada
        if field == 'Nenhum':
            return
            
        # Sempre limpa o plotter para garantir atualização correta
        self.plotter.clear()
        
        # Theme colormaps
        theme_opts = {
            'light': {'bg': 'white', 'density_cmap': 'turbo', 'vel_cmap': 'plasma', 'vort_cmap': 'cool'},
            'dark':  {'bg': 'black', 'density_cmap': 'viridis', 'vel_cmap': 'plasma', 'vort_cmap': 'cool'}
        }
        t = theme_opts.get(self.theme, theme_opts['light'])
        self.plotter.set_background(t['bg'])
        
        # Mask solids if needed
        grid = self.grid
        if not self.show_solids and 'solid' in grid.array_names:
            mask = grid['solid'] < 1
            grid = grid.extract_points(mask)
            
        # Se há campo escalar selecionado, adiciona primeiro
        scalar_field = self.scalar_field_box.currentText()
        if scalar_field != 'Nenhum':
            self.add_scalar_field_to_plot(grid, scalar_field, t)
        
        # Adiciona sólidos se necessário
        self.add_solids_to_plot()
        
        # Seed points: regular grid
        bounds = grid.bounds
        x = np.linspace(bounds[0], bounds[1], 10)
        y = np.linspace(bounds[2], bounds[3], 10)
        z = np.linspace(bounds[4], bounds[5], 10)
        seed = pv.PolyData(np.array(np.meshgrid(x, y, z)).reshape(3, -1).T)

        if field == 'velocity' and 'velocity' in grid.array_names:
            vectors = 'velocity'
            cmap = t['vel_cmap']
            bar_title = 'Velocidade'
        elif field == 'vorticity' and 'vorticity' in grid.array_names:
            vectors = 'vorticity'
            cmap = t['vort_cmap']
            bar_title = 'Vorticidade'
        else:
            if scalar_field == 'Nenhum':
                self.plotter.add_mesh(grid, color='gray')
            self.plotter.reset_camera()
            return

        try:
            stream = grid.streamlines_from_source(
                seed,
                vectors=vectors,
                max_time=200.0,
                initial_step_length=1.0,
                terminal_speed=1e-5,
                integrator_type=45,
            )
            # Color by magnitude
            if vectors in grid.array_names:
                vec = grid[vectors]
                mag = np.linalg.norm(vec, axis=1)
                stream['mag'] = mag[:stream.n_points]
                vmin, vmax = np.nanmin(mag), np.nanmax(mag)
            else:
                vmin, vmax = None, None
                
            self.plotter.add_mesh(
                stream.tube(radius=0.2),
                scalars='mag',
                cmap=cmap,
                clim=(vmin, vmax) if vmin is not None and vmax is not None else None,
                show_scalar_bar=True,
                scalar_bar_args={'title': bar_title},
            )
        except Exception as e:
            self.plotter.add_text(f'Erro ao calcular streamlines: {e}', color='red')
        self.plotter.reset_camera()

    def open_file(self):
        fname, _ = QtWidgets.QFileDialog.getOpenFileName(self, 'Abrir arquivo VTK', '', 'VTK Files (*.vtk)')
        if fname:
            self.progress.setVisible(True)
            self.plotter.clear()
            loading_text = self.plotter.add_text('Carregando...', position=(0.5, 0.5), font_size=24, color='black')
            QtWidgets.QApplication.processEvents()
            self.filename = fname
            self.grid = pv.read(fname)
            self.compute_derived_fields()
            
            # Se ambos os campos estão em "Nenhum", mostra densidade por padrão
            if self.current_field == 'Nenhum' and self.stream_field_box.currentText() == 'Nenhum':
                if 'density' in self.grid.array_names:
                    self.scalar_field_box.setCurrentText('density')
                else:
                    # Se não há densidade, mostra pelo menos os sólidos se existirem
                    self.show_field(self.current_field)
            else:
                self.show_field(self.current_field)
                
            self.progress.setVisible(False)
            self.plotter.remove_actor(loading_text)

    def compute_derived_fields(self):
        # Vorticity (magnitude)
        if 'velocity' in self.grid.array_names and 'vorticity' not in self.grid.array_names:
            vel = self.grid['velocity']
            # Central differences for vorticity (approximate)
            shape = self.grid.dimensions
            vorticity = np.zeros_like(vel)
            try:
                # Only works for structured grid
                vel3d = vel.reshape((-1, 3))
                nx, ny, nz = shape
                vel3d = vel3d.reshape((nx, ny, nz, 3))
                for i in range(1, nx-1):
                    for j in range(1, ny-1):
                        for k in range(1, nz-1):
                            du_dy = (vel3d[i, j+1, k, 0] - vel3d[i, j-1, k, 0]) / 2
                            du_dz = (vel3d[i, j, k+1, 0] - vel3d[i, j, k-1, 0]) / 2
                            dv_dx = (vel3d[i+1, j, k, 1] - vel3d[i-1, j, k, 1]) / 2
                            dv_dz = (vel3d[i, j, k+1, 1] - vel3d[i, j, k-1, 1]) / 2
                            dw_dx = (vel3d[i+1, j, k, 2] - vel3d[i-1, j, k, 2]) / 2
                            dw_dy = (vel3d[i, j+1, k, 2] - vel3d[i, j-1, k, 2]) / 2
                            vort_x = dw_dy - dv_dz
                            vort_y = du_dz - dw_dx
                            vort_z = dv_dx - du_dy
                            vorticity[i*ny*nz + j*nz + k] = [vort_x, vort_y, vort_z]
                vort_mag = np.linalg.norm(vorticity, axis=1)
                self.grid['vorticity'] = vorticity
                self.grid['vorticity_mag'] = vort_mag
            except Exception:
                pass
        # Q-criterion (if not present)
        if 'q_criterion' not in self.grid.array_names:
            # Placeholder: set to zeros
            self.grid['q_criterion'] = np.zeros(self.grid.n_points)

    def show_field(self, field):
        if self.grid is None:
            return
        self.current_field = field
        
        # Atualiza o combobox se necessário (evita loop infinito)
        if self.scalar_field_box.currentText() != field:
            self.scalar_field_box.blockSignals(True)
            self.scalar_field_box.setCurrentText(field)
            self.scalar_field_box.blockSignals(False)
        
        self.plotter.clear()
        
        # Se "Nenhum" foi selecionado, não mostra nenhum campo escalar
        if field == 'Nenhum':
            self.plotter.reset_camera()
            return
            
        # Theme colormaps
        theme_opts = {
            'light': {'bg': 'white', 'density_cmap': 'turbo', 'vel_cmap': 'plasma', 'vort_cmap': 'cool'},
            'dark':  {'bg': 'black', 'density_cmap': 'viridis', 'vel_cmap': 'plasma', 'vort_cmap': 'cool'}
        }
        t = theme_opts.get(self.theme, theme_opts['light'])
        self.plotter.set_background(t['bg'])
        # Mask solids if needed
        grid = self.grid
        if not self.show_solids and 'solid' in grid.array_names:
            mask = grid['solid'] < 1
            grid = grid.extract_points(mask)
        
        if field == 'velocity' and 'velocity' in grid.array_names:
            vel = grid['velocity']
            vel_mag = np.linalg.norm(vel, axis=1)
            vmin, vmax = np.nanmin(vel_mag), np.nanmax(vel_mag)
            self.plotter.add_mesh(grid, scalars=vel_mag, cmap=t['vel_cmap'], show_scalar_bar=True, clim=(vmin, vmax), scalar_bar_args={'title': 'Velocidade'})
        elif field == 'vorticity' and 'vorticity' in grid.array_names:
            vort = grid['vorticity']
            vort_mag = np.linalg.norm(vort, axis=1)
            vmin, vmax = np.nanmin(vort_mag), np.nanmax(vort_mag)
            self.plotter.add_mesh(grid, scalars=vort_mag, cmap=t['vort_cmap'], show_scalar_bar=True, clim=(vmin, vmax), scalar_bar_args={'title': 'Vorticidade'})
        elif field == 'q_criterion' and 'q_criterion' in grid.array_names:
            # Isosurface Q=0 (ou Q>0) para destacar regiões vorticosas
            try:
                # Extrai isosuperfície Q=0
                surf = grid.contour(isosurfaces=[0.0], scalars='q_criterion')
                self.plotter.add_mesh(
                    surf,
                    color='red',
                    opacity=0.7,
                    name='Q-criterion isosurface',
                    show_scalar_bar=False
                )
                # Opcional: mostrar o grid de fundo em cinza claro
                self.plotter.add_mesh(grid, color='lightgray', opacity=0.1)
            except Exception as e:
                self.plotter.add_text(f'Erro ao gerar isosuperfície Q: {e}', color='red')
        elif field == 'density' and 'density' in grid.array_names:
            dens = grid['density']
            dmin, dmax = np.nanmin(dens), np.nanmax(dens)
            self.plotter.add_mesh(grid, scalars='density', cmap=t['density_cmap'], show_scalar_bar=True, clim=(dmin, dmax), scalar_bar_args={'title': 'Densidade'})
        else:
            self.plotter.add_mesh(grid, color='gray')
        
        # Adiciona sólidos se necessário
        self.add_solids_to_plot()
        
        self.plotter.reset_camera()

    def update_ui_style(self, style_name):
        """Atualiza o estilo visual da interface"""
        styles = {
            'Default': self.get_default_style(),
            'Dark': self.get_dark_style()
        }
        
        if style_name in styles:
            self.setStyleSheet(styles[style_name])

    def get_default_style(self):
        return """
            QMainWindow {
                background-color: #f0f0f0;
                color: #333333;
                font-family: 'Segoe UI', Arial, sans-serif;
                font-size: 9pt;
            }
            QFrame {
                background-color: #f5f5f5;
                border: 1px solid #d0d0d0;
                border-radius: 5px;
            }
            QMenuBar {
                background-color: #f0f0f0;
                color: #333333;
                border-bottom: 1px solid #d0d0d0;
            }
            QMenuBar::item {
                background-color: #f0f0f0;
                color: #333333;
                padding: 4px 8px;
            }
            QMenuBar::item:selected {
                background-color: #e1e1e1;
            }
            QMenu {
                background-color: white;
                color: #333333;
                border: 1px solid #d0d0d0;
            }
            QMenu::item {
                background-color: white;
                color: #333333;
                padding: 6px 12px;
            }
            QMenu::item:selected {
                background-color: #e1e1e1;
            }
            QMenu::separator {
                height: 1px;
                background-color: #d0d0d0;
                margin: 2px 0;
            }
            QPushButton {
                background-color: #e1e1e1;
                border: 1px solid #adadad;
                border-radius: 4px;
                padding: 6px 12px;
                color: #333333;
                font-weight: bold;
            }
            QPushButton:hover {
                background-color: #d4d4d4;
            }
            QPushButton:pressed {
                background-color: #bcbcbc;
            }
            QComboBox {
                background-color: white;
                border: 1px solid #adadad;
                border-radius: 3px;
                padding: 3px 8px;
                color: #333333;
            }
            QComboBox QAbstractItemView {
                background-color: white;
                color: #333333;
                selection-background-color: #e1e1e1;
            }
            QLabel {
                color: #333333;
                font-weight: 500;
            }
            QCheckBox {
                color: #333333;
            }
            QCheckBox::indicator {
                background-color: white;
                border: 1px solid #adadad;
            }
            QCheckBox::indicator:checked {
                background-color: #4a9eff;
            }
            QProgressBar {
                background-color: white;
                border: 1px solid #adadad;
                border-radius: 3px;
            }
            QProgressBar::chunk {
                background-color: #4a9eff;
                border-radius: 2px;
            }
        """

    def get_dark_style(self):
        return """
            QMainWindow {
                background-color: #2b2b2b;
                color: #ffffff;
                font-family: 'Segoe UI', Arial, sans-serif;
                font-size: 9pt;
            }
            QFrame {
                background-color: #3c3c3c;
                border: 1px solid #555555;
                border-radius: 5px;
            }
            QMenuBar {
                background-color: #2b2b2b;
                color: #ffffff;
                border-bottom: 1px solid #555555;
            }
            QMenuBar::item {
                background-color: #2b2b2b;
                color: #ffffff;
                padding: 4px 8px;
            }
            QMenuBar::item:selected {
                background-color: #505050;
            }
            QMenu {
                background-color: #3c3c3c;
                color: #ffffff;
                border: 1px solid #555555;
            }
            QMenu::item {
                background-color: #3c3c3c;
                color: #ffffff;
                padding: 6px 12px;
            }
            QMenu::item:selected {
                background-color: #505050;
            }
            QMenu::separator {
                height: 1px;
                background-color: #555555;
                margin: 2px 0;
            }
            QPushButton {
                background-color: #505050;
                border: 1px solid #707070;
                border-radius: 4px;
                padding: 6px 12px;
                color: #ffffff;
                font-weight: bold;
            }
            QPushButton:hover {
                background-color: #606060;
            }
            QPushButton:pressed {
                background-color: #404040;
            }
            QComboBox {
                background-color: #404040;
                border: 1px solid #707070;
                border-radius: 3px;
                padding: 3px 8px;
                color: #ffffff;
            }
            QComboBox QAbstractItemView {
                background-color: #404040;
                color: #ffffff;
                selection-background-color: #606060;
            }
            QComboBox::drop-down {
                border: none;
            }
            QComboBox::down-arrow {
                image: none;
                border: 2px solid #ffffff;
                width: 6px;
                height: 6px;
            }
            QLabel {
                color: #ffffff;
                font-weight: 500;
            }
            QCheckBox {
                color: #ffffff;
            }
            QCheckBox::indicator {
                background-color: #404040;
                border: 1px solid #707070;
            }
            QCheckBox::indicator:checked {
                background-color: #4a9eff;
            }
            QProgressBar {
                background-color: #404040;
                border: 1px solid #707070;
                border-radius: 3px;
            }
            QProgressBar::chunk {
                background-color: #4a9eff;
                border-radius: 2px;
            }
        """

    def apply_custom_font(self, font_family="Segoe UI", font_size=9):
        """Aplica uma fonte customizada para toda a interface"""
        font = QtCore.QFont(font_family, font_size)
        self.setFont(font)
        
    def set_custom_colors(self, primary_color="#6366f1", secondary_color="#f3f4f6", text_color="#374151"):
        """Permite personalizar cores específicas"""
        custom_style = f"""
            QPushButton {{
                background-color: {primary_color};
                color: white;
                border: none;
                border-radius: 6px;
                padding: 8px 16px;
                font-weight: 600;
            }}
            QPushButton:hover {{
                background-color: {primary_color}dd;
            }}
            QFrame {{
                background-color: {secondary_color};
            }}
            QLabel {{
                color: {text_color};
            }}
            QCheckBox {{
                color: {text_color};
            }}
        """
        self.setStyleSheet(self.styleSheet() + custom_style)

if __name__ == '__main__':
    app = QtWidgets.QApplication(sys.argv)
    viewer = VTKViewer()
    viewer.show()
    sys.exit(app.exec_())
