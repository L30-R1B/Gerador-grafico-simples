use plotters::prelude::*;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let nome_arquivo = input.trim().to_string();
    input.clear();

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let cor_barras = input.trim().to_lowercase();
    let cor_barras = match cor_barras.as_str() {
        "red" => RED,
        "blue" => BLUE,
        "green" => GREEN,
        "yellow" => YELLOW,
        "black" => BLACK,
        _ => {
            eprintln!("Cor desconhecida, usando preto por padrão.");
            BLACK
        }
    };
    input.clear();

    // Nome do gráfico
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let nome_grafico = input.trim().to_string();
    input.clear();

    // Nome do eixo X
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let nome_eixo_x = input.trim().to_string();
    input.clear();

    // Nome do eixo Y
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let nome_eixo_y = input.trim().to_string();
    input.clear();

    // Tamanho dos dados
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let tamanho: usize = input.trim().parse()?;
    input.clear();

    // Leitura dos dados
    let mut pontos = Vec::with_capacity(tamanho);
    io::stdout().flush()?;
    for _ in 0..tamanho {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let mut iter = input.split_whitespace();
        let x: f64 = iter.next().ok_or("Erro ao ler o valor de x")?.parse()?;
        let y: f64 = iter.next().ok_or("Erro ao ler o valor de y")?.parse()?;
        pontos.push((x, y));
    }

    // Configuração do gráfico
    let root_area = BitMapBackend::new(&nome_arquivo, (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(nome_grafico, ("sans-serif", 50))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .build_cartesian_2d(
            pontos.iter().map(|(x, _)| *x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..
            pontos.iter().map(|(x, _)| *x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            pontos.iter().map(|(_, y)| *y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..
            pontos.iter().map(|(_, y)| *y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
        )?;

    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc(&nome_eixo_x)
        .y_desc(&nome_eixo_y)
        .draw()?;

    // Adicionando as barras
    chart.draw_series(
        pontos.into_iter().map(|(x, y)| {
            Rectangle::new(
                [(x - 0.5, 0.0), (x + 0.5, y)], 
                cor_barras.filled()
            )
        })
    )?
    .label("Dados")
    .legend(|(x, y)| Rectangle::new(
        [(x - 15, y - 15), (x + 15, y + 15)], 
        cor_barras.filled()
    ));

    // Configurando a legenda
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    Ok(())
}
