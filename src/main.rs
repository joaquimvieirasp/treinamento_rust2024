use chrono::{DateTime, Local};

fn main() {
    let agora: DateTime<Local> = Local::now();

    println!("Entrada: {}", agora.format("%d-%m-%Y %H:%M:%S"));

    //Simular a saída
    let saida: DateTime<Local> = agora
        + chrono::Duration::days(0)
        + chrono::Duration::hours(0)
        + chrono::Duration::minutes(502);

    println!("Saída: {}", saida.format("%d-%m-%y %H:%M:%S"));

    // Calcular a quantidade de horas trabalhadas
    let duracao: chrono::TimeDelta = saida.signed_duration_since(agora);
    let horas_totais: i64 = duracao.num_hours();
    let minutos_totais: i64 = duracao.num_minutes() - horas_totais * 60;

    println!(
        "Total de horas trabalhadas: {} horas e {} minutos",
        horas_totais, minutos_totais
    );
}
