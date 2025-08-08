"""
Gerador de relatório detalhado de validação para CappuSim.

Analisa os arquivos CSV gerados pelo validate.py e produz:
- Relatório em texto com estatísticas detalhadas
- Gráficos comparativos entre todos os casos
- Análise de convergência e precisão

Uso:
    python relatorio_validacao.py
"""

import os
import glob
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from datetime import datetime


def configurar_matplotlib():
    """Configura matplotlib para relatórios científicos."""
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
    
    # Tamanho para relatório científico
    plt.rcParams['figure.figsize'] = (10.0, 5.5)
    plt.rcParams['figure.dpi'] = 150
    plt.rcParams['savefig.dpi'] = 300
    plt.rcParams['savefig.bbox'] = 'tight'
    plt.rcParams['savefig.pad_inches'] = 0.1


def carregar_dados_csv(arquivo):
    """Carrega dados de um arquivo CSV de validação."""
    try:
        df = pd.read_csv(arquivo)
        return df
    except Exception as e:
        print(f"Erro ao carregar {arquivo}: {e}")
        return None


def calcular_metricas(df):
    """Calcula métricas estatísticas dos erros."""
    erro_abs = df['erro_absoluto']
    erro_rel = df['erro_relativo_pct']
    
    metricas = {
        'erro_rms': np.sqrt(np.mean(erro_abs**2)),
        'erro_max': np.max(erro_abs),
        'erro_medio': np.mean(erro_abs),
        'erro_std': np.std(erro_abs),
        'erro_rel_max': np.max(erro_rel),
        'erro_rel_medio': np.mean(erro_rel),
        'coef_correlacao': np.corrcoef(df['u_simulacao'], df['u_referencia'])[0, 1]
    }
    
    return metricas


def gerar_grafico_comparativo():
    """Gera gráfico comparativo de todos os casos em formato científico."""
    arquivos_csv = glob.glob('results/dados_*.csv')
    
    if not arquivos_csv:
        print("Nenhum arquivo CSV de dados encontrado em results/")
        return
    
    n_casos = len(arquivos_csv)
    
    # Layout baseado no número de casos
    if n_casos <= 3:
        fig, axes = plt.subplots(1, n_casos, figsize=(12.0, 4.0))
        if n_casos == 1:
            axes = [axes]
    elif n_casos <= 6:
        fig, axes = plt.subplots(2, 3, figsize=(12.0, 6.0))
        axes = axes.flatten()
    else:
        # Para mais casos
        rows = int(np.ceil(n_casos / 3))
        fig, axes = plt.subplots(rows, 3, figsize=(12.0, 4.0 * rows))
        axes = axes.flatten()
    
    # Ajustar espaçamento
    plt.subplots_adjust(hspace=0.4, wspace=0.4, left=0.08, right=0.95,
                       top=0.90, bottom=0.15)
    
    colors = ['#1f77b4', '#d62728', '#2ca02c', '#ff7f0e', '#9467bd', '#8c564b']
    
    for i, arquivo in enumerate(arquivos_csv):
        if i >= len(axes):
            break
            
        df = carregar_dados_csv(arquivo)
        if df is None:
            continue
        
        ax = axes[i]
        color = colors[i % len(colors)]
        
        # Gráfico principal com estilo científico
        ax.plot(df['posicao'], df['u_simulacao'], 'o-', color=color,
               label='Simulação LBM', markersize=2, linewidth=1.2,
               markerfacecolor='white', markeredgewidth=0.8)
        ax.plot(df['posicao'], df['u_referencia'], '--', color='black',
               label='Referência', linewidth=1.5, alpha=0.8)
        
        # Configuração do subplot
        caso = arquivo.replace('results/dados_', '').replace('.csv', '').replace('_', ' ').title()
        ax.set_title(caso, fontweight='bold', fontsize=10)
        ax.set_xlabel('Posição y/H', fontweight='bold', fontsize=9)
        ax.set_ylabel('Velocidade u/U', fontweight='bold', fontsize=9)
        ax.legend(fontsize=7, loc='best')
        ax.grid(True, alpha=0.3)
        
        # Adicionar erro RMS
        metricas = calcular_metricas(df)
        ax.text(0.05, 0.95, f'RMS: {metricas["erro_rms"]:.1e}', 
               transform=ax.transAxes, fontsize=8,
               bbox=dict(boxstyle="round,pad=0.1", facecolor="white", alpha=0.8))
        
        ax.tick_params(labelsize=7)
    
    # Remover subplots vazios
    for j in range(i+1, len(axes)):
        fig.delaxes(axes[j])
    
    # Título geral
    fig.suptitle('Validação CappuSim: Comparação Simulação vs. Referência', 
                fontweight='bold', fontsize=14, y=0.98)
    
    plt.savefig('results/comparacao_todos_casos.png', dpi=300, bbox_inches='tight',
               facecolor='white', edgecolor='none', pad_inches=0.1)
    plt.show()
    print("Gráfico comparativo salvo: results/comparacao_todos_casos.png")


def gerar_grafico_erros():
    """Gera gráfico de análise de erros em formato científico."""
    arquivos_csv = glob.glob('results/dados_*.csv')
    
    if not arquivos_csv:
        return
    
    # Figura científica compacta
    fig = plt.figure(figsize=(10.0, 5.0))
    gs = fig.add_gridspec(1, 2, width_ratios=[1.5, 1], wspace=0.3,
                         left=0.08, right=0.95, top=0.88, bottom=0.15)
    
    ax1 = fig.add_subplot(gs[0])
    ax2 = fig.add_subplot(gs[1])
    
    casos = []
    erros_rms = []
    erros_max = []
    colors = ['#1f77b4', '#d62728', '#2ca02c', '#ff7f0e', '#9467bd', '#8c564b']
    
    for i, arquivo in enumerate(arquivos_csv):
        df = carregar_dados_csv(arquivo)
        if df is None:
            continue
        
        caso = arquivo.replace('results/dados_', '').replace('.csv', '').replace('_', ' ').title()
        casos.append(caso)
        
        metricas = calcular_metricas(df)
        erros_rms.append(metricas['erro_rms'])
        erros_max.append(metricas['erro_max'])
        
        # Gráfico de erro por posição
        color = colors[i % len(colors)]
        ax1.plot(df['posicao'], df['erro_absoluto'], 
                label=caso, linewidth=1.5, color=color)
    
    # Configuração do gráfico de distribuição de erro
    ax1.set_xlabel('Posição y/H', fontweight='bold')
    ax1.set_ylabel('Erro Absoluto', fontweight='bold')
    ax1.set_title('Distribuição de Erro por Posição', fontweight='bold', fontsize=11)
    ax1.legend(fontsize=7, loc='best')
    ax1.grid(True, alpha=0.3)
    ax1.tick_params(labelsize=8)
    
    # Gráfico de barras com erros RMS
    x_pos = np.arange(len(casos))
    bars = ax2.bar(x_pos, erros_rms, alpha=0.7, color='steelblue', 
                   edgecolor='navy', linewidth=0.8)
    
    ax2.set_xlabel('Casos de Teste', fontweight='bold')
    ax2.set_ylabel('Erro RMS', fontweight='bold')
    ax2.set_title('Erro RMS por Caso', fontweight='bold', fontsize=11)
    ax2.set_xticks(x_pos)
    ax2.set_xticklabels([caso.replace(' ', '\n') for caso in casos], 
                       fontsize=7, ha='center')
    ax2.grid(True, alpha=0.3, axis='y')
    ax2.tick_params(labelsize=8)
    
    # Adicionar valores nas barras
    for i, bar in enumerate(bars):
        height = bar.get_height()
        ax2.text(bar.get_x() + bar.get_width()/2., height + height*0.02,
                f'{height:.1e}', ha='center', va='bottom', fontsize=7,
                fontweight='bold')
    
    # Título geral
    fig.suptitle('Análise de Precisão - CappuSim LBM Solver', 
                fontweight='bold', fontsize=14)
    
    plt.savefig('results/analise_erros.png', dpi=300, bbox_inches='tight',
               facecolor='white', edgecolor='none', pad_inches=0.1)
    plt.show()
    print("Gráfico de erros salvo: results/analise_erros.png")


def gerar_tabela_resumo():
    """Gera tabela resumo visual dos resultados."""
    arquivos_csv = glob.glob('results/dados_*.csv')
    
    if not arquivos_csv:
        return
    
    # Coletar dados
    dados_tabela = []
    for arquivo in arquivos_csv:
        df = carregar_dados_csv(arquivo)
        if df is None:
            continue
        
        caso = arquivo.replace('results/dados_', '').replace('.csv', '')
        metricas = calcular_metricas(df)
        
        # Classificação de qualidade
        if metricas['erro_rms'] < 0.001:
            qualidade = "Excelente"
            cor = '#2ca02c'  # Verde
        elif metricas['erro_rms'] < 0.01:
            qualidade = "Muito Boa"
            cor = '#ff7f0e'  # Laranja
        elif metricas['erro_rms'] < 0.1:
            qualidade = "Boa"
            cor = '#1f77b4'  # Azul
        else:
            qualidade = "Regular"
            cor = '#d62728'  # Vermelho
        
        dados_tabela.append({
            'caso': caso.replace('_', ' ').title(),
            'erro_rms': metricas['erro_rms'],
            'correlacao': metricas['coef_correlacao'],
            'qualidade': qualidade,
            'cor': cor
        })
    
    # Ordenar por erro RMS
    dados_tabela.sort(key=lambda x: x['erro_rms'])
    
    # Criar figura de tabela
    fig, ax = plt.subplots(figsize=(10.0, 3.0 + 0.3 * len(dados_tabela)))
    ax.axis('tight')
    ax.axis('off')
    
    # Preparar dados da tabela
    headers = ['Ranking', 'Caso de Teste', 'Erro RMS', 'Correlação R²', 'Avaliação']
    dados_formatados = []
    
    for i, dados in enumerate(dados_tabela, 1):
        dados_formatados.append([
            f"{i}º",
            dados['caso'],
            f"{dados['erro_rms']:.2e}",
            f"{dados['correlacao']:.4f}",
            dados['qualidade']
        ])
    
    # Criar tabela
    tabela = ax.table(cellText=dados_formatados, colLabels=headers,
                     cellLoc='center', loc='center',
                     bbox=[0, 0, 1, 1])
    
    # Estilizar tabela
    tabela.auto_set_font_size(False)
    tabela.set_fontsize(9)
    tabela.scale(1, 2.0)
    
    # Colorir header
    for i in range(len(headers)):
        tabela[(0, i)].set_facecolor('#4472C4')
        tabela[(0, i)].set_text_props(weight='bold', color='white')
    
    # Colorir linhas alternadas e qualidade
    for i, dados in enumerate(dados_tabela, 1):
        cor_linha = '#f0f0f0' if i % 2 == 0 else 'white'
        for j in range(len(headers)):
            tabela[(i, j)].set_facecolor(cor_linha)
            if j == 4:  # Coluna de qualidade
                tabela[(i, j)].set_text_props(weight='bold', color=dados['cor'])
    
    plt.title('Resumo de Validação - Ranking por Precisão', 
             fontweight='bold', fontsize=14, pad=20)
    
    plt.savefig('results/tabela_resumo_validacao.png', dpi=300, bbox_inches='tight',
               facecolor='white', edgecolor='none', pad_inches=0.1)
    plt.show()
    print("Tabela resumo salva: results/tabela_resumo_validacao.png")


def gerar_relatorio_texto():
    """Gera relatório detalhado em texto."""
    arquivos_csv = glob.glob('results/dados_*.csv')
    
    if not arquivos_csv:
        print("Nenhum arquivo CSV encontrado para relatório")
        return
    
    with open('results/relatorio_validacao.txt', 'w', encoding='utf-8') as f:
        f.write("="*80 + "\n")
        f.write("RELATÓRIO DE VALIDAÇÃO - CAPPUSIM LBM SOLVER\n")
        f.write("="*80 + "\n")
        f.write(f"Data de geração: {datetime.now().strftime('%d/%m/%Y %H:%M:%S')}\n")
        f.write(f"Número de casos analisados: {len(arquivos_csv)}\n\n")
        
        resultados_gerais = []
        
        for arquivo in arquivos_csv:
            df = carregar_dados_csv(arquivo)
            if df is None:
                continue
            
            caso = arquivo.replace('results/dados_', '').replace('.csv', '')
            metricas = calcular_metricas(df)
            
            f.write(f"CASO: {caso.upper()}\n")
            f.write("-" * 40 + "\n")
            f.write(f"Número de pontos: {len(df)}\n")
            f.write(f"Erro RMS: {metricas['erro_rms']:.8f}\n")
            f.write(f"Erro máximo: {metricas['erro_max']:.8f}\n")
            f.write(f"Erro médio: {metricas['erro_medio']:.8f}\n")
            f.write(f"Desvio padrão do erro: {metricas['erro_std']:.8f}\n")
            f.write(f"Erro relativo máximo: {metricas['erro_rel_max']:.2f}%\n")
            f.write(f"Erro relativo médio: {metricas['erro_rel_medio']:.2f}%\n")
            f.write(f"Coeficiente de correlação: {metricas['coef_correlacao']:.6f}\n")
            
            # Análise de qualidade
            if metricas['erro_rms'] < 0.001:
                qualidade = "EXCELENTE"
                simbolo = "★★★★★"
            elif metricas['erro_rms'] < 0.01:
                qualidade = "MUITO BOA"
                simbolo = "★★★★☆"
            elif metricas['erro_rms'] < 0.1:
                qualidade = "BOA"
                simbolo = "★★★☆☆"
            else:
                qualidade = "REGULAR"
                simbolo = "★★☆☆☆"
            
            f.write(f"Avaliação da precisão: {qualidade} {simbolo}\n\n")
            
            resultados_gerais.append({
                'caso': caso,
                'erro_rms': metricas['erro_rms'],
                'correlacao': metricas['coef_correlacao']
            })
        
        # Resumo geral
        f.write("RESUMO GERAL\n")
        f.write("="*40 + "\n")
        
        erros_rms = [r['erro_rms'] for r in resultados_gerais]
        correlacoes = [r['correlacao'] for r in resultados_gerais]
        
        f.write(f"Erro RMS médio: {np.mean(erros_rms):.8f}\n")
        f.write(f"Erro RMS mínimo: {np.min(erros_rms):.8f}\n")
        f.write(f"Erro RMS máximo: {np.max(erros_rms):.8f}\n")
        f.write(f"Correlação média: {np.mean(correlacoes):.6f}\n")
        
        # Classificação por precisão
        f.write(f"\nRANKING POR PRECISÃO (Erro RMS):\n")
        f.write("-" * 40 + "\n")
        resultados_ordenados = sorted(resultados_gerais, key=lambda x: x['erro_rms'])
        for i, resultado in enumerate(resultados_ordenados, 1):
            if resultado['erro_rms'] < 0.001:
                status = "EXCELENTE ✨"
            elif resultado['erro_rms'] < 0.01:
                status = "MUITO BOA ✅"
            elif resultado['erro_rms'] < 0.1:
                status = "BOA ✅"
            else:
                status = "REGULAR ⚠️"
                
            f.write(f"{i}. {resultado['caso'].upper():<15}: {resultado['erro_rms']:.6f} - {status}\n")
        
        f.write(f"\nCRITÉRIOS DE AVALIAÇÃO:\n")
        f.write("-" * 40 + "\n")
        f.write(f"★★★★★ Excelente:  Erro RMS < 0.001\n")
        f.write(f"★★★★☆ Muito Boa:  Erro RMS < 0.01\n")
        f.write(f"★★★☆☆ Boa:       Erro RMS < 0.1\n")
        f.write(f"★★☆☆☆ Regular:    Erro RMS ≥ 0.1\n")
        f.write(f"\nCORRELAÇÃO:\n")
        f.write(f"- R² > 0.99: Excelente concordância\n")
        f.write(f"- R² > 0.95: Boa concordância\n")
        f.write(f"- R² > 0.90: Concordância aceitável\n")
    
    print("Relatório salvo: results/relatorio_validacao.txt")


def main():
    """Função principal do gerador de relatórios."""
    configurar_matplotlib()
    
    print("Gerando relatório de validação científico...")
    
    # Verificar se existem dados
    arquivos_csv = glob.glob('results/dados_*.csv')
    if not arquivos_csv:
        print("Erro: Nenhum arquivo de dados encontrado em results/.")
        print("Execute primeiro: python validate.py --salvar-dados")
        return
    
    print(f"Encontrados {len(arquivos_csv)} arquivos de dados")
    
    # Gerar componentes do relatório
    print("Gerando relatório em texto...")
    gerar_relatorio_texto()
    
    print("Gerando gráficos comparativos...")
    gerar_grafico_comparativo()
    
    print("Gerando análise de erros...")
    gerar_grafico_erros()
    
    print("Gerando tabela resumo...")
    gerar_tabela_resumo()
    
    print("\n" + "="*60)
    print("RELATÓRIO CIENTÍFICO COMPLETO GERADO:")
    print("="*60)
    print("📄 results/relatorio_validacao.txt")
    print("📊 results/comparacao_todos_casos.png")
    print("📈 results/analise_erros.png")
    print("📋 results/tabela_resumo_validacao.png")
    print("="*60)


if __name__ == '__main__':
    main()
