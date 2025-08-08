"""
Script de validação automatizado para CappuSim LBM solver.

Analisa automaticamente os arquivos VTK em validation/data e gera:
- Gráficos de comparação entre simulação e solução analítica
- Análise de erros (absoluto e relativo)
- Exportação de dados para CSV
- Relatórios de validação

Casos suportados:
- Couette flow (escoamento entre placas paralelas)
- Poiseuille flow (escoamento em canal)
- Lid-driven cavity (cavidade com tampa móvel)

Uso:
    python validate.py                    # Analisa todos os casos
    python validate.py --caso couette     # Analisa apenas Couette
    python validate.py --salvar-dados    # Salva dados em CSV
"""

import argparse
import os
import glob
import numpy as np
import matplotlib.pyplot as plt
import pyvista as pv
import csv
from pathlib import Path


def configurar_matplotlib():
    """Configura matplotlib para gráficos científicos em formato A4."""
    # Configurações para publicação científica
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
    plt.rcParams['axes.grid'] = True
    plt.rcParams['grid.alpha'] = 0.3
    plt.rcParams['grid.linewidth'] = 0.5
    plt.rcParams['axes.linewidth'] = 0.8
    
    plt.rcParams['figure.figsize'] = (9.0, 4.5)
    plt.rcParams['figure.dpi'] = 150  # Reduzido para display
    plt.rcParams['savefig.dpi'] = 300  # Alto para salvar
    plt.rcParams['savefig.bbox'] = 'tight'
    plt.rcParams['savefig.pad_inches'] = 0.1 


def carregar_vtk(caminho_arquivo):
    """
    Carrega dados de um arquivo VTK.
    
    Returns:
        tuple: (velocidades, dimensões, densidade) ou None se erro
    """
    try:
        grid = pv.read(caminho_arquivo)
        
        if 'velocity' not in grid.array_names:
            print(f"Aviso: Campo 'velocity' não encontrado em {caminho_arquivo}")
            return None
            
        vel = np.asarray(grid['velocity'])
        
        # Obter dimensões
        dims = getattr(grid, 'dimensions', None)
        if dims is None:
            try:
                dims = grid.GetDimensions()
            except:
                print(f"Erro: Não foi possível obter dimensões de {caminho_arquivo}")
                return None
                
        nx, ny, nz = int(dims[0]), int(dims[1]), int(dims[2])
        
        # Carregar densidade se disponível
        densidade = None
        if 'density' in grid.array_names:
            densidade = np.asarray(grid['density'])
            
        return vel, (nx, ny, nz), densidade
        
    except Exception as e:
        print(f"Erro ao carregar {caminho_arquivo}: {e}")
        return None


def extrair_perfil_central(vel, dims, direcao='y'):
    """
    Extrai perfil de velocidade na linha central.
    
    Args:
        vel: Array de velocidades (N, 3)
        dims: Dimensões da grade (nx, ny, nz)
        direcao: Direção do perfil ('x', 'y', ou 'z')
        
    Returns:
        tuple: (posições, velocidades_u)
    """
    nx, ny, nz = dims
    
    if direcao == 'y':
        # Perfil ao longo de y (centro em x e z)
        x_centro = nx // 2
        z_centro = nz // 2
        perfil = np.zeros(ny)
        posicoes = np.arange(ny) / max(ny - 1, 1)
        
        for y in range(ny):
            i = z_centro * (nx * ny) + y * nx + x_centro
            perfil[y] = vel[i, 0]  # componente u_x
            
    elif direcao == 'x':
        # Perfil ao longo de x (centro em y e z)
        y_centro = ny // 2
        z_centro = nz // 2
        perfil = np.zeros(nx)
        posicoes = np.arange(nx) / max(nx - 1, 1)
        
        for x in range(nx):
            i = z_centro * (nx * ny) + y_centro * nx + x
            perfil[x] = vel[i, 0]  # componente u_x
            
    else:
        raise ValueError(f"Direção '{direcao}' não suportada")
        
    return posicoes, perfil


def analitico_couette(y, u_topo=0.1):
    """Solução analítica para escoamento de Couette: u(y) = U_topo * y"""
    return u_topo * y


def analitico_poiseuille(y, u_max=0.05, altura=1.0):
    """Solução analítica para escoamento de Poiseuille: u(y) = U_Max * (4y/H)(1 - y/H)"""
    y_norm = y / altura
    return u_max * 4 * y_norm * (1 - y_norm)


def analitico_lid_driven_cavity(y, re=100):
    """
    Dados de referência de Ghia et al. (1982) para cavidade com tampa móvel.
    Retorna velocidade u na linha central vertical.
    """
    # Dados para Re=100 (linha central vertical, u vs y)
    dados_ghia = {
        100: {
            'y': [0.0000, 0.0547, 0.0625, 0.0703, 0.1016, 0.1719, 0.2813, 0.4531, 
                  0.5000, 0.6172, 0.7344, 0.8516, 0.9531, 0.9609, 0.9688, 0.9766, 1.0000],
            'u': [0.0000, -0.03717, -0.04192, -0.04775, -0.06434, -0.10150, -0.15662, -0.21090,
                  -0.20581, -0.13641, 0.00332, 0.23151, 0.68717, 0.73722, 0.78871, 0.84123, 1.00000]
        },
        400: {
            'y': [0.0000, 0.0547, 0.0625, 0.0703, 0.1016, 0.1719, 0.2813, 0.4531,
                  0.5000, 0.6172, 0.7344, 0.8516, 0.9531, 0.9609, 0.9688, 0.9766, 1.0000],
            'u': [0.0000, -0.08186, -0.09266, -0.10338, -0.14612, -0.24299, -0.32726, -0.17119,
                  -0.11477, 0.02135, 0.16256, 0.29093, 0.55892, 0.61756, 0.68439, 0.75837, 1.00000]
        },
        1000: {
            'y': [0.0000, 0.0547, 0.0625, 0.0703, 0.1016, 0.1719, 0.2813, 0.4531,
                  0.5000, 0.6172, 0.7344, 0.8516, 0.9531, 0.9609, 0.9688, 0.9766, 1.0000],
            'u': [0.0000, -0.18109, -0.20196, -0.22220, -0.29730, -0.38289, -0.27805, -0.10648,
                  -0.06080, 0.05702, 0.18719, 0.33304, 0.46547, 0.51117, 0.57492, 0.65928, 1.00000]
        }
    }
    
    if re not in dados_ghia:
        print(f"Aviso: Re={re} não disponível, usando Re=100")
        re = 100
        
    y_ref = np.array(dados_ghia[re]['y'])
    u_ref = np.array(dados_ghia[re]['u'])
    
    return np.interp(y, y_ref, u_ref)


def calcular_erros(u_sim, u_ref):
    """
    Calcula erros absoluto e relativo entre simulação e referência.
    
    Returns:
        tuple: (erro_absoluto, erro_relativo, erro_rms)
    """
    erro_abs = np.abs(u_sim - u_ref)
    
    # Evitar divisão por zero no erro relativo
    u_ref_safe = np.where(np.abs(u_ref) < 1e-10, 1e-10, u_ref)
    erro_rel = np.abs((u_sim - u_ref) / u_ref_safe) * 100
    
    # Erro RMS (Root Mean Square)
    erro_rms = np.sqrt(np.mean((u_sim - u_ref)**2))
    
    return erro_abs, erro_rel, erro_rms


def plotar_comparacao(pos, u_sim, u_ref, titulo, nome_arquivo=None, info_adicional=None):
    """
    Plota comparação entre simulação e solução de referência com layout científico compacto.
    
    Args:
        pos: Posições normalizadas
        u_sim: Velocidades da simulação
        u_ref: Velocidades de referência
        titulo: Título do gráfico
        nome_arquivo: Nome do arquivo para salvar
        info_adicional: Dict com informações extras (Re, nx, ny, etc.)
    """
    # Calcular erros
    erro_abs, erro_rel, erro_rms = calcular_erros(u_sim, u_ref)
    
    # Criar figura mais larga
    fig = plt.figure(figsize=(9.0, 4.5))
    
    # Layout com margens maiores para acomodar labels
    gs = fig.add_gridspec(2, 2, height_ratios=[2.5, 1], width_ratios=[2.5, 1], 
                         hspace=0.4, wspace=0.5,  # Aumentado wspace
                         left=0.12, right=0.95, top=0.90, bottom=0.15)  # Ajustado margens
    
    # Gráfico principal
    ax1 = fig.add_subplot(gs[0, :])
    
    # Plotar dados com marcadores menores
    ax1.plot(pos, u_sim, 'o-', color='#1f77b4', 
             label='Simulação LBM', markersize=2.5, linewidth=1.2, 
             markerfacecolor='white', markeredgewidth=0.8)
    ax1.plot(pos, u_ref, '--', color='#d62728', 
             label='Referência', linewidth=1.5)
    
    ax1.set_xlabel('Posição y/H', fontweight='bold')
    ax1.set_ylabel('Velocidade u/U', fontweight='bold')
    ax1.set_title(titulo, fontweight='bold', fontsize=11, pad=15)
    
    # Legend mais compacta
    legend = ax1.legend(loc='best', frameon=True, fancybox=False, 
                       shadow=False, fontsize=8)
    legend.get_frame().set_alpha(0.9)
    legend.get_frame().set_linewidth(0.5)
    
    # Grid
    ax1.grid(True, alpha=0.3, linestyle='-', linewidth=0.5)
    ax1.set_axisbelow(True)
    
    # Gráfico de erro
    ax2 = fig.add_subplot(gs[1, 0])
    ax2.plot(pos, erro_abs, 'r-', linewidth=1.2)
    ax2.fill_between(pos, erro_abs, alpha=0.3, color='red')
    ax2.set_xlabel('Posição y/H', fontsize=9)
    ax2.set_ylabel('Erro Abs.', fontsize=9)
    ax2.set_title(f'Erro (RMS={erro_rms:.1e})', fontsize=10)
    ax2.grid(True, alpha=0.3)
    
    # Caixa de estatísticas mais compacta
    ax3 = fig.add_subplot(gs[1, 1])
    ax3.axis('off')
    
    # Texto mais conciso
    stats_text = f"Validação\n"
    stats_text += f"RMS: {erro_rms:.1e}\n"
    stats_text += f"Max: {np.max(erro_abs):.1e}\n"
    stats_text += f"R²: {np.corrcoef(u_sim, u_ref)[0,1]**2:.3f}\n"
    
    if info_adicional:
        stats_text += f"\nSetup\n"
        for key, value in list(info_adicional.items())[:2]:  # Só 2 primeiros
            stats_text += f"{key}: {value}\n"
    
    ax3.text(0.05, 0.95, stats_text, transform=ax3.transAxes, 
             fontsize=8, verticalalignment='top', fontfamily='monospace',
             bbox=dict(boxstyle="round,pad=0.2", facecolor="lightgray", alpha=0.8))
    
    # Salvar se especificado
    if nome_arquivo:
        results_dir = "results"
        os.makedirs(results_dir, exist_ok=True)
        nome_arquivo_completo = os.path.join(results_dir, nome_arquivo)
        
        plt.savefig(nome_arquivo_completo, dpi=300, bbox_inches='tight', 
                   facecolor='white', edgecolor='none', pad_inches=0.1)
        print(f"Gráfico salvo: {nome_arquivo_completo}")
    
    plt.show()
    return erro_rms


def plotar_comparacao_multiplo(resultados_dict, nome_arquivo=None):
    """
    Cria gráfico comparativo compacto com múltiplos casos.
    
    Args:
        resultados_dict: Dict com {nome_caso: (pos, u_sim, u_ref, info)}
        nome_arquivo: Nome do arquivo para salvar
    """
    n_casos = len(resultados_dict)
    
    # Layout compacto baseado no número de casos
    if n_casos <= 2:
        fig, axes = plt.subplots(1, n_casos, figsize=(8.0, 3.5))  # Aumentado largura
        if n_casos == 1:
            axes = [axes]
    elif n_casos <= 4:
        fig, axes = plt.subplots(2, 2, figsize=(8.0, 5.5))  # Aumentado largura
        axes = axes.flatten()
    else:
        # Para mais casos, usar grid 2x3
        fig, axes = plt.subplots(2, 3, figsize=(10.0, 5.5))  # Aumentado largura
        axes = axes.flatten()
    
    # Ajustar espaçamento com margens maiores
    plt.subplots_adjust(hspace=0.4, wspace=0.4, left=0.12, right=0.95,  # Aumentado left e wspace
                       top=0.90, bottom=0.15)  # Ajustado margens
    
    colors = ['#1f77b4', '#d62728', '#2ca02c', '#ff7f0e', '#9467bd', '#8c564b']
    
    for i, (nome_caso, dados) in enumerate(resultados_dict.items()):
        pos, u_sim, u_ref, info = dados
        ax = axes[i]
        
        # Plot com marcadores menores
        ax.plot(pos, u_sim, 'o-', color=colors[i % len(colors)], 
               label='Sim.', markersize=1.5, linewidth=1.2,
               markerfacecolor='white', markeredgewidth=0.6)
        ax.plot(pos, u_ref, '--', color='black', 
               label='Ref.', linewidth=1.2, alpha=0.8)
        
        # Configurações compactas
        ax.set_xlabel('y/H', fontsize=9)
        ax.set_ylabel('u/U', fontsize=9)
        ax.set_title(nome_caso, fontweight='bold', fontsize=10)
        ax.grid(True, alpha=0.3)
        ax.legend(fontsize=8, loc='best')
        
        # Erro RMS menor
        erro_rms = np.sqrt(np.mean((u_sim - u_ref)**2))
        ax.text(0.05, 0.95, f'{erro_rms:.1e}', 
               transform=ax.transAxes, fontsize=8,
               bbox=dict(boxstyle="round,pad=0.1", facecolor="white", alpha=0.8))
        
        # Ajustar ticks
        ax.tick_params(labelsize=8)
    
    # Remover subplots vazios
    for j in range(i + 1, len(axes)):
        fig.delaxes(axes[j])
    
    if nome_arquivo:
        results_dir = "results"
        os.makedirs(results_dir, exist_ok=True)
        nome_arquivo_completo = os.path.join(results_dir, nome_arquivo)
        
        plt.savefig(nome_arquivo_completo, dpi=300, bbox_inches='tight',
                   facecolor='white', edgecolor='none', pad_inches=0.1)
        print(f"Gráfico comparativo salvo: {nome_arquivo_completo}")
    
    plt.show()


# Atualizar as funções de análise para usar info_adicional
def analisar_couette(caminho_arquivo, salvar_dados=False):
    """Analisa caso de escoamento de Couette."""
    print(f"\n=== Analisando Couette: {caminho_arquivo} ===")
    
    dados = carregar_vtk(caminho_arquivo)
    if dados is None:
        return
        
    vel, dims, _ = dados
    nx, ny, nz = dims
    
    # Extrair perfil central
    pos, u_sim = extrair_perfil_central(vel, dims, 'y')
    
    # Estimar velocidade do topo pela simulação
    u_topo = np.max(u_sim)
    u_ref = analitico_couette(pos, u_topo)
    
    # Informações adicionais
    info = {
        'Nx × Ny': f'{nx} × {ny}',
        'U_topo': f'{u_topo:.4f}',
        'Tipo': 'Couette Flow'
    }
    
    # Plotar
    titulo = f'Validação: Escoamento de Couette'
    nome_grafico = 'validacao_couette.png' if salvar_dados else None
    erro_rms = plotar_comparacao(pos, u_sim, u_ref, titulo, nome_grafico, info)
    
    if salvar_dados:
        salvar_dados_csv(pos, u_sim, u_ref, 'dados_couette.csv')
    
    print(f"Erro RMS: {erro_rms:.6f}")
    return erro_rms


def analisar_poiseuille(caminho_arquivo, salvar_dados=False):
    """Analisa caso de escoamento de Poiseuille."""
    print(f"\n=== Analisando Poiseuille: {caminho_arquivo} ===")
    
    dados = carregar_vtk(caminho_arquivo)
    if dados is None:
        return
        
    vel, dims, _ = dados
    nx, ny, nz = dims
    
    # Extrair perfil central
    pos, u_sim = extrair_perfil_central(vel, dims, 'y')
    
    # Estimar velocidade máxima
    u_max = np.max(u_sim)
    u_ref = analitico_poiseuille(pos, u_max)
    
    # Informações adicionais
    info = {
        'Nx × Ny': f'{nx} × {ny}',
        'U_max': f'{u_max:.4f}',
        'Tipo': 'Poiseuille Flow'
    }
    
    # Plotar
    titulo = f'Validação: Escoamento de Poiseuille'
    nome_grafico = 'validacao_poiseuille.png' if salvar_dados else None
    erro_rms = plotar_comparacao(pos, u_sim, u_ref, titulo, nome_grafico, info)
    
    if salvar_dados:
        salvar_dados_csv(pos, u_sim, u_ref, 'dados_poiseuille.csv')
    
    print(f"Erro RMS: {erro_rms:.6f}")
    return erro_rms


def analisar_lid_driven_cavity(caminho_arquivo, re=100, salvar_dados=False):
    """Analisa caso de cavidade com tampa móvel."""
    print(f"\n=== Analisando Lid-driven Cavity Re={re}: {caminho_arquivo} ===")
    
    dados = carregar_vtk(caminho_arquivo)
    if dados is None:
        return
        
    vel, dims, _ = dados
    nx, ny, nz = dims
    
    # Extrair perfil central vertical
    pos, u_sim = extrair_perfil_central(vel, dims, 'y')
    
    # Normalizar pela velocidade da tampa (assumindo u=1 no topo)
    u_sim_norm = u_sim / max(abs(u_sim.max()), abs(u_sim.min()), 1e-10)
    
    # Solução de referência
    u_ref = analitico_lid_driven_cavity(pos, re)
    
    # Informações adicionais
    info = {
        'Reynolds': f'{re}',
        'Nx × Ny': f'{nx} × {ny}',
        'Tipo': 'Lid-driven Cavity',
        'Ref.': 'Ghia et al. (1982)'
    }
    
    # Plotar
    titulo = f'Validação: Cavidade com Tampa Móvel (Re = {re})'
    nome_grafico = f'validacao_cavity_re{re}.png' if salvar_dados else None
    erro_rms = plotar_comparacao(pos, u_sim_norm, u_ref, titulo, nome_grafico, info)
    
    if salvar_dados:
        salvar_dados_csv(pos, u_sim_norm, u_ref, f'dados_cavity_re{re}.csv')
    
    print(f"Erro RMS: {erro_rms:.6f}")
    return erro_rms


def detectar_caso_por_nome(nome_arquivo):
    """Detecta o tipo de caso baseado no nome do arquivo."""
    nome_lower = nome_arquivo.lower()
    
    if 'couette' in nome_lower:
        return 'couette'
    elif 'poiseuille' in nome_lower:
        return 'poiseuille'
    elif 'lid' in nome_lower or 'cavity' in nome_lower:
        # Extrair número de Reynolds se presente
        import re
        match = re.search(r're(\d+)', nome_lower)
        if match:
            return 'cavity', int(match.group(1))
        else:
            return 'cavity', 100  # Padrão Re=100
    else:
        return 'desconhecido'


def main():
    parser = argparse.ArgumentParser(description='Validação automatizada CappuSim')
    parser.add_argument('--caso', choices=['couette', 'poiseuille', 'cavity', 'todos'], 
                       default='todos', help='Tipo de caso para analisar')
    parser.add_argument('--pasta-dados', default='validation/data', 
                       help='Pasta com arquivos VTK')
    parser.add_argument('--salvar-dados', action='store_true', 
                       help='Salvar dados em arquivos CSV')
    args = parser.parse_args()
    
    configurar_matplotlib()
    
    # Encontrar arquivos VTK
    pasta_dados = Path(args.pasta_dados)
    if not pasta_dados.exists():
        print(f"Erro: Pasta {pasta_dados} não encontrada")
        return
    
    arquivos_vtk = list(pasta_dados.glob('*.vtk'))
    if not arquivos_vtk:
        print(f"Nenhum arquivo VTK encontrado em {pasta_dados}")
        return
    
    print(f"Encontrados {len(arquivos_vtk)} arquivos VTK")
    
    resultados = {}
    
    for arquivo in arquivos_vtk:
        nome = arquivo.name
        deteccao = detectar_caso_por_nome(nome)
        
        if isinstance(deteccao, tuple):
            tipo_caso, re = deteccao
        else:
            tipo_caso = deteccao
            re = None
        
        if args.caso != 'todos' and args.caso != tipo_caso:
            continue
            
        try:
            if tipo_caso == 'couette':
                erro = analisar_couette(str(arquivo), args.salvar_dados)
            elif tipo_caso == 'poiseuille':
                erro = analisar_poiseuille(str(arquivo), args.salvar_dados)
            elif tipo_caso == 'cavity':
                erro = analisar_lid_driven_cavity(str(arquivo), re or 100, args.salvar_dados)
            else:
                print(f"Tipo de caso não reconhecido para {nome}")
                continue
                
            if erro is not None:
                resultados[nome] = erro
                
        except Exception as e:
            print(f"Erro ao processar {nome}: {e}")
    
    # Resumo dos resultados
    print(f"\n{'='*50}")
    print("RESUMO DOS RESULTADOS")
    print(f"{'='*50}")
    
    if resultados:
        for nome, erro in resultados.items():
            print(f"{nome:30s} | Erro RMS: {erro:.6f}")
        
        erro_medio = np.mean(list(resultados.values()))
        print(f"\nErro RMS médio: {erro_medio:.6f}")
    else:
        print("Nenhum resultado obtido.")


if __name__ == '__main__':
    main()
