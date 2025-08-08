"""
Script para visualização de campos de velocidade e densidade das simulações CappuSim.

Gera visualizações científicas dos campos escalares e vetoriais:
- Contornos de magnitude de velocidade
- Campos vetoriais (streamlines)
- Campos de densidade
- Comparações lado a lado

Uso:
    python plot_simulations.py                    # Plota todos os VTKs na pasta
    python plot_simulations.py --arquivo xyz.vtk  # Plota arquivo específico
    python plot_simulations.py --tipo velocity    # Só campos de velocidade
    python plot_simulations.py --tipo density     # Só campos de densidade
"""

import os
import glob
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.patches as patches
from matplotlib.colors import Normalize
import pyvista as pv
import argparse
from pathlib import Path


def configurar_matplotlib():
    """Configura matplotlib para visualizações científicas."""
    plt.rcParams['font.family'] = 'serif'
    plt.rcParams['font.serif'] = ['Times New Roman', 'DejaVu Serif']
    plt.rcParams['font.size'] = 9
    plt.rcParams['axes.titlesize'] = 11
    plt.rcParams['axes.labelsize'] = 10
    plt.rcParams['xtick.labelsize'] = 8
    plt.rcParams['ytick.labelsize'] = 8
    plt.rcParams['legend.fontsize'] = 8
    plt.rcParams['figure.titlesize'] = 12
    
    # Grid e estilo
    plt.rcParams['axes.grid'] = False  # Desabilitado para campos
    plt.rcParams['axes.linewidth'] = 0.8
    
    # Tamanho para visualizações
    plt.rcParams['figure.figsize'] = (12.0, 6.0)
    plt.rcParams['figure.dpi'] = 150
    plt.rcParams['savefig.dpi'] = 300
    plt.rcParams['savefig.bbox'] = 'tight'
    plt.rcParams['savefig.pad_inches'] = 0.1


def carregar_dados_vtk(arquivo_vtk):
    """
    Carrega dados de velocidade e densidade de um arquivo VTK.
    
    Returns:
        dict: Dicionário com campos 'velocidade', 'densidade', 'dimensoes', 'info'
    """
    try:
        mesh = pv.read(arquivo_vtk)
        
        # Extrair dimensões
        dims = mesh.dimensions
        nx, ny, nz = dims
        
        # Extrair dados
        dados = {}
        
        # Velocidade
        if 'velocity' in mesh.array_names:
            vel_data = mesh['velocity']
            dados['velocidade'] = vel_data.reshape((nx, ny, nz, 3))
        elif 'Velocity' in mesh.array_names:
            vel_data = mesh['Velocity']
            dados['velocidade'] = vel_data.reshape((nx, ny, nz, 3))
        else:
            print(f"Aviso: Campo de velocidade não encontrado em {arquivo_vtk}")
            dados['velocidade'] = None
        
        # Densidade
        if 'density' in mesh.array_names:
            dens_data = mesh['density']
            dados['densidade'] = dens_data.reshape((nx, ny, nz))
        elif 'Density' in mesh.array_names:
            dens_data = mesh['Density']
            dados['densidade'] = dens_data.reshape((nx, ny, nz))
        else:
            print(f"Aviso: Campo de densidade não encontrado em {arquivo_vtk}")
            dados['densidade'] = None
        
        # Informações
        dados['dimensoes'] = (nx, ny, nz)
        dados['info'] = {
            'arquivo': os.path.basename(arquivo_vtk),
            'dimensoes': f"{nx}×{ny}×{nz}",
            'n_pontos': nx * ny * nz
        }
        
        return dados
        
    except Exception as e:
        print(f"Erro ao carregar {arquivo_vtk}: {e}")
        return None


def plotar_campo_velocidade(dados, nome_arquivo=None):
    """
    Plota campo de velocidade com contornos e streamlines.
    """
    if dados['velocidade'] is None:
        print("Dados de velocidade não disponíveis")
        return
    
    vel = dados['velocidade']
    nx, ny, nz = dados['dimensoes']
    
    # Para 2D, usar fatia central em z
    if nz == 1:
        vel_2d = vel[:, :, 0, :]
    else:
        vel_2d = vel[:, :, nz//2, :]
    
    # Componentes de velocidade - TESTANDO ORIENTAÇÃO CORRETA
    u = vel_2d[:, :, 0]  # Componente x
    v = vel_2d[:, :, 1]  # Componente y
    
    # Magnitude da velocidade
    mag = np.sqrt(u**2 + v**2)
    
    # Coordenadas normalizadas - INVERTER ORDEM
    y = np.linspace(0, 1, nx)  # Agora y usa nx
    x = np.linspace(0, 1, ny)  # Agora x usa ny
    X, Y = np.meshgrid(x, y)
    
    # NÃO transpor os dados
    u_plot = u
    v_plot = v
    mag_plot = mag
    
    # Criar figura com layout científico
    fig = plt.figure(figsize=(12.0, 5.0))
    gs = fig.add_gridspec(1, 2, width_ratios=[1, 1], wspace=0.3,
                         left=0.08, right=0.95, top=0.88, bottom=0.15)
    
    # === SUBPLOT 1: Contornos de magnitude ===
    ax1 = fig.add_subplot(gs[0])
    
    # Contornos preenchidos
    levels = np.linspace(0, np.max(mag_plot), 20)
    cs1 = ax1.contourf(X, Y, mag_plot, levels=levels, cmap='viridis', alpha=0.8)
    
    # Contornos de linha
    cs1_lines = ax1.contour(X, Y, mag_plot, levels=levels[::2], colors='black', 
                           linewidths=0.5, alpha=0.6)
    
    # Colorbar
    cbar1 = plt.colorbar(cs1, ax=ax1, shrink=0.8, aspect=20)
    cbar1.set_label('|V| (lattice units)', fontweight='bold')
    cbar1.ax.tick_params(labelsize=8)
    
    ax1.set_xlabel('x/L', fontweight='bold')
    ax1.set_ylabel('y/H', fontweight='bold')
    ax1.set_title('Magnitude da Velocidade', fontweight='bold', fontsize=11)
    ax1.set_aspect('equal')
    ax1.tick_params(labelsize=8)
    
    # === SUBPLOT 2: Streamlines ===
    ax2 = fig.add_subplot(gs[1])
    
    # Streamlines com densidade adaptativa
    densidade_stream = min(2.0, max(0.5, 50/min(nx, ny)))
    
    # Fundo com contornos suaves
    cs2 = ax2.contourf(X, Y, mag_plot, levels=levels, cmap='plasma', alpha=0.6)
    
    # Streamlines - usar dados sem transposição
    streams = ax2.streamplot(X, Y, u_plot, v_plot, density=densidade_stream, 
                            color='white', linewidth=1.0,
                            arrowsize=1.2, arrowstyle='->')
    
    # Colorbar
    cbar2 = plt.colorbar(cs2, ax=ax2, shrink=0.8, aspect=20)
    cbar2.set_label('|V| (lattice units)', fontweight='bold')
    cbar2.ax.tick_params(labelsize=8)
    
    ax2.set_xlabel('x/L', fontweight='bold')
    ax2.set_ylabel('y/H', fontweight='bold')
    ax2.set_title('Linhas de Corrente', fontweight='bold', fontsize=11)
    ax2.set_aspect('equal')
    ax2.tick_params(labelsize=8)
    
    # Título geral
    case_name = dados['info']['arquivo'].replace('.vtk', '').replace('_', ' ').title()
    fig.suptitle(f'Campo de Velocidade: {case_name}', fontweight='bold', fontsize=14)
    
    # Salvar
    if nome_arquivo:
        plt.savefig(nome_arquivo, dpi=300, bbox_inches='tight',
                   facecolor='white', edgecolor='none', pad_inches=0.1)
        print(f"Campo de velocidade salvo: {nome_arquivo}")
    
    plt.show()


def plotar_campo_densidade(dados, nome_arquivo=None):
    """
    Plota campo de densidade.
    """
    if dados['densidade'] is None:
        print("Dados de densidade não disponíveis")
        return
    
    dens = dados['densidade']
    nx, ny, nz = dados['dimensoes']
    
    # Para 2D, usar fatia central em z
    if nz == 1:
        dens_2d = dens[:, :, 0]
    else:
        dens_2d = dens[:, :, nz//2]
    
    # Coordenadas normalizadas - INVERTER ORDEM
    y = np.linspace(0, 1, nx)  # Agora y usa nx
    x = np.linspace(0, 1, ny)  # Agora x usa ny
    X, Y = np.meshgrid(x, y)
    
    # NÃO transpor dados
    dens_plot = dens_2d
    
    # Criar figura
    fig, ax = plt.subplots(1, 1, figsize=(8.0, 6.0))
    
    # Ajustar margens
    plt.subplots_adjust(left=0.12, right=0.88, top=0.88, bottom=0.15)
    
    # Contornos de densidade
    dens_min, dens_max = np.min(dens_plot), np.max(dens_plot)
    dens_range = dens_max - dens_min
    
    if dens_range > 1e-10:  # Se há variação significativa
        levels = np.linspace(dens_min, dens_max, 25)
        cs = ax.contourf(X, Y, dens_plot, levels=levels, cmap='RdYlBu_r', alpha=0.9)
        
        # Contornos de linha
        cs_lines = ax.contour(X, Y, dens_plot, levels=levels[::3], 
                             colors='black', linewidths=0.5, alpha=0.7)
        
        # Labels dos contornos (apenas alguns)
        ax.clabel(cs_lines, inline=True, fontsize=7, fmt='%.3f')
    else:
        # Densidade uniforme
        cs = ax.contourf(X, Y, dens_plot, levels=20, cmap='Blues', alpha=0.8)
    
    # Colorbar
    cbar = plt.colorbar(cs, ax=ax, shrink=0.8, aspect=20)
    cbar.set_label('Densidade ρ (lattice units)', fontweight='bold')
    cbar.ax.tick_params(labelsize=8)
    
    # Configurações
    ax.set_xlabel('x/L', fontweight='bold')
    ax.set_ylabel('y/H', fontweight='bold')
    ax.set_aspect('equal')
    ax.tick_params(labelsize=8)
    
    # Título
    case_name = dados['info']['arquivo'].replace('.vtk', '').replace('_', ' ').title()
    ax.set_title(f'Campo de Densidade: {case_name}', fontweight='bold', fontsize=12, pad=15)
    
    # Adicionar informações
    info_text = f"Dimensões: {dados['info']['dimensoes']}\n"
    info_text += f"ρ min: {dens_min:.6f}\n"
    info_text += f"ρ max: {dens_max:.6f}\n"
    info_text += f"Δρ: {dens_range:.2e}"
    
    ax.text(0.02, 0.98, info_text, transform=ax.transAxes, 
            fontsize=8, verticalalignment='top', fontfamily='monospace',
            bbox=dict(boxstyle="round,pad=0.3", facecolor="white", alpha=0.8))
    
    # Salvar
    if nome_arquivo:
        plt.savefig(nome_arquivo, dpi=300, bbox_inches='tight',
                   facecolor='white', edgecolor='none', pad_inches=0.1)
        print(f"Campo de densidade salvo: {nome_arquivo}")
    
    plt.show()


def plotar_campos_combinados(dados, nome_arquivo=None):
    """
    Plota velocidade e densidade lado a lado.
    """
    if dados['velocidade'] is None and dados['densidade'] is None:
        print("Nenhum campo disponível para plotagem")
        return
    
    vel = dados['velocidade']
    dens = dados['densidade']
    nx, ny, nz = dados['dimensoes']
    
    # Coordenadas - INVERTER ORDEM
    y = np.linspace(0, 1, nx)  # Agora y usa nx  
    x = np.linspace(0, 1, ny)  # Agora x usa ny
    X, Y = np.meshgrid(x, y)
    
    # Determinar layout
    n_plots = sum([dados['velocidade'] is not None, dados['densidade'] is not None])
    
    if n_plots == 2:
        fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12.0, 5.0))
        axes = [ax1, ax2]
    else:
        fig, ax = plt.subplots(1, 1, figsize=(8.0, 6.0))
        axes = [ax]
    
    plt.subplots_adjust(hspace=0.3, wspace=0.3, left=0.08, right=0.92,
                       top=0.88, bottom=0.15)
    
    plot_idx = 0
    
    # === VELOCIDADE ===
    if vel is not None:
        ax = axes[plot_idx]
        
        # Preparar dados 2D
        if nz == 1:
            vel_2d = vel[:, :, 0, :]
        else:
            vel_2d = vel[:, :, nz//2, :]
        
        u = vel_2d[:, :, 0]  # Componente x
        v = vel_2d[:, :, 1]  # Componente y
        mag = np.sqrt(u**2 + v**2)
        
        # NÃO transpor dados
        u_plot = u
        v_plot = v
        mag_plot = mag
        
        # Plot
        levels = np.linspace(0, np.max(mag_plot), 20)
        cs = ax.contourf(X, Y, mag_plot, levels=levels, cmap='viridis', alpha=0.8)
        
        # Streamlines seletivas
        densidade_stream = min(1.5, 30/min(nx, ny))
        ax.streamplot(X, Y, u_plot, v_plot, density=densidade_stream, 
                     color='white', linewidth=0.8,
                     arrowsize=1.0, arrowstyle='->')
        
        # Colorbar
        cbar = plt.colorbar(cs, ax=ax, shrink=0.8, aspect=20)
        cbar.set_label('|V|', fontweight='bold')
        cbar.ax.tick_params(labelsize=8)
        
        ax.set_xlabel('x/L', fontweight='bold')
        ax.set_ylabel('y/H', fontweight='bold')
        ax.set_title('Campo de Velocidade', fontweight='bold', fontsize=11)
        ax.set_aspect('equal')
        ax.tick_params(labelsize=8)
        
        plot_idx += 1
    
    # === DENSIDADE ===
    if dens is not None:
        ax = axes[plot_idx]
        
        # Preparar dados 2D
        if nz == 1:
            dens_2d = dens[:, :, 0]
        else:
            dens_2d = dens[:, :, nz//2]
        
        # NÃO transpor dados
        dens_plot = dens_2d
        
        # Plot
        dens_min, dens_max = np.min(dens_plot), np.max(dens_plot)
        levels = np.linspace(dens_min, dens_max, 20)
        cs = ax.contourf(X, Y, dens_plot, levels=levels, cmap='RdYlBu_r', alpha=0.9)
        
        # Colorbar
        cbar = plt.colorbar(cs, ax=ax, shrink=0.8, aspect=20)
        cbar.set_label('ρ', fontweight='bold')
        cbar.ax.tick_params(labelsize=8)
        
        ax.set_xlabel('x/L', fontweight='bold')
        ax.set_ylabel('y/H', fontweight='bold')
        ax.set_title('Campo de Densidade', fontweight='bold', fontsize=11)
        ax.set_aspect('equal')
        ax.tick_params(labelsize=8)
    
    # Título geral
    case_name = dados['info']['arquivo'].replace('.vtk', '').replace('_', ' ').title()
    fig.suptitle(f'Campos de Simulação: {case_name}', fontweight='bold', fontsize=14)
    
    # Salvar
    if nome_arquivo:
        plt.savefig(nome_arquivo, dpi=300, bbox_inches='tight',
                   facecolor='white', edgecolor='none', pad_inches=0.1)
        print(f"Campos combinados salvos: {nome_arquivo}")
    
    plt.show()


def processar_arquivo(arquivo_vtk, tipo_plot='ambos', salvar=True):
    """
    Processa um arquivo VTK individual.
    
    Args:
        arquivo_vtk: Caminho para o arquivo VTK
        tipo_plot: 'velocity', 'density', 'ambos'
        salvar: Se deve salvar as figuras
    """
    print(f"\nProcessando: {arquivo_vtk}")
    
    # Carregar dados
    dados = carregar_dados_vtk(arquivo_vtk)
    if dados is None:
        return
    
    # Preparar nomes de arquivo
    base_name = os.path.splitext(os.path.basename(arquivo_vtk))[0]
    results_dir = "../results"
    os.makedirs(results_dir, exist_ok=True)
    
    # Plotar conforme solicitado
    if tipo_plot in ['velocity', 'ambos'] and dados['velocidade'] is not None:
        nome_arquivo = os.path.join(results_dir, f"campo_velocidade_{base_name}.png") if salvar else None
        plotar_campo_velocidade(dados, nome_arquivo)
    
    if tipo_plot in ['density', 'ambos'] and dados['densidade'] is not None:
        nome_arquivo = os.path.join(results_dir, f"campo_densidade_{base_name}.png") if salvar else None
        plotar_campo_densidade(dados, nome_arquivo)
    
    if tipo_plot == 'ambos':
        nome_arquivo = os.path.join(results_dir, f"campos_combinados_{base_name}.png") if salvar else None
        plotar_campos_combinados(dados, nome_arquivo)


def main():
    """Função principal."""
    parser = argparse.ArgumentParser(description='Visualizador de campos CappuSim')
    parser.add_argument('--arquivo', type=str, help='Arquivo VTK específico')
    parser.add_argument('--tipo', choices=['velocity', 'density', 'ambos'], 
                       default='ambos', help='Tipo de campo a plotar')
    parser.add_argument('--no-save', action='store_true', 
                       help='Não salvar figuras (apenas mostrar)')
    parser.add_argument('--pasta-dados', default='data', 
                       help='Pasta com arquivos VTK (padrão: data)')
    
    args = parser.parse_args()
    
    configurar_matplotlib()
    
    # Determinar arquivos a processar
    if args.arquivo:
        if os.path.exists(args.arquivo):
            arquivos = [args.arquivo]
        else:
            print(f"Erro: Arquivo {args.arquivo} não encontrado")
            return
    else:
        # Buscar todos os VTKs na pasta de dados
        pasta_dados = Path(args.pasta_dados)
        if not pasta_dados.exists():
            print(f"Erro: Pasta {pasta_dados} não encontrada")
            return
        
        arquivos = list(pasta_dados.glob("*.vtk"))
        if not arquivos:
            print(f"Nenhum arquivo VTK encontrado em {pasta_dados}")
            return
        
        # Converter para strings
        arquivos = [str(arquivo) for arquivo in arquivos]
    
    print(f"Encontrados {len(arquivos)} arquivo(s) VTK")
    print(f"Tipo de plot: {args.tipo}")
    print(f"Salvar figuras: {not args.no_save}")
    
    # Processar arquivos
    for arquivo in arquivos:
        processar_arquivo(arquivo, args.tipo, not args.no_save)
    
    print("\nProcessamento concluído!")
    if not args.no_save:
        print("Figuras salvas em: ../results/")


if __name__ == '__main__':
    main()