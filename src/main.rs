use chrono::{Datelike, Duration, NaiveDate};

fn calcular_fecha_regreso(
    fecha_inicio: &str,
    dias_vacaciones: i32,
    dias_no_laborables: Vec<&str>,
) -> String {
    // Obtengo la fecha actual a partir de "parsear" la fecha de inicio dada
    let mut fecha_actual = NaiveDate::parse_from_str(fecha_inicio, "%d/%m/%Y").unwrap();
    let mut dias_faltantes = 0;

    while dias_faltantes < dias_vacaciones {
        // A la fecha actual le sumo un día en cada iteración
        // hasta llegar al total de días de disfrute.
        fecha_actual += Duration::days(1);

        // Importante: Obtengo el día de la semana
        let dia_semana = fecha_actual.weekday();

        // Agregamos un día faltante para el final del disfrute vacacional
        // a partir de verificar si el vector de días no laborables no
        // contiene a fecha actual y si dia de la semana no es fin
        // de semana (No sumamos sab ni dom para el conteo de días hábiles; además de los días no laborables pasados).
        if ! dias_no_laborables.contains(&fecha_actual.format("%d/%m/%Y").to_string().as_str())
            && dia_semana.num_days_from_monday() < 5 { // Monday = 0, Tuesday = 1, ...
            dias_faltantes += 1;
        }
    }

    fecha_actual.format("%d/%m/%Y").to_string()
}

fn main() {
    let fecha_inicio = "25/09/2023";
    let dias_vacaciones = 30;
    let dias_no_laborables = vec!["12/10/2023", "30/10/2023", "06/11/2023"];

    let fecha_regreso = calcular_fecha_regreso(fecha_inicio, dias_vacaciones, dias_no_laborables);

    println!("Debes regresar a trabajar el {}", fecha_regreso);
}
