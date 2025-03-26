# Configurações gerais
set terminal pngcairo enhanced size 1600,1200 font "Arial,12"
set datafile separator ","
set pm3d map
set size ratio -1  # Mantém proporção quadrada

# Paleta Inferno com gradiente suave
set palette defined (\
    0 '#000004',\
    1 '#1b0c41',\
    2 '#4a0c6b',\
    3 '#781c6d',\
    4 '#b63679',\
    5 '#ed5768',\
    6 '#fb8861',\
    7 '#fec287',\
    8 '#f7feb2'\
)

# Criar pasta de resultados (Linux/macOS)
!mkdir -p results

# Lista todos os arquivos CSV no diretório output
filelist = system("find output -name '*.csv' | sort")

# Processa cada arquivo
do for [file in filelist] {
    # Extrai nome do arquivo sem extensão
    filename = system('basename '.file.' .csv')
    
    # Configura saída
    set output 'results/'.filename.'.png'
    
    # Layout multiplot (2x2)
    set multiplot layout 2,2 columnsfirst title filename font ",16"
    
    # 1. Gráfico de Densidade (colunas 1:2:4)
    set title "Densidade (ρ)" font ",14"
    set cblabel "ρ" offset 2,0
    set autoscale
    plot file using 1:2:4 with image title ""
    
    # 2. Mapa de Velocidade Escalar (colunas 1:2:7)
    set title "Mapa de Velocidade Escalar" font ",14"
    set cblabel "Velocidade" offset 2,0
    set autoscale
    plot file using 1:2:(sqrt($5**2 + $6**2 + $7**2)) with image title ""
    
    # 3. Vorticidade (colunas 1:2:8)
    set title "Vorticidade" font ",14"
    set cblabel "ω" offset 2,0
    set autoscale
    plot file using 1:2:8 with image title ""
    
    # 4. Critério Q (colunas 1:2:9)
    set title "Critério Q" font ",14"
    set cblabel "Q" offset 2,0
    set autoscale
    plot file using 1:2:9 with image title ""
    
    unset multiplot
    clear
}

print "Processamento concluído! Gráficos salvos em: results/"
