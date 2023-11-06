from datetime import datetime, timedelta

def calcular_fecha_regreso(fecha_inicio, dias_vacaciones, dias_no_laborables):
    fecha_actual = datetime.strptime(fecha_inicio, '%d/%m/%Y')
    dias_faltantes = 0

    while dias_faltantes < dias_vacaciones:
        fecha_actual += timedelta(days=1)
        dia_semana = fecha_actual.weekday()

        if fecha_actual.strftime('%d/%m/%Y') not in dias_no_laborables and dia_semana < 5:
            dias_faltantes += 1

    return fecha_actual.strftime('%d/%m/%Y')

# Definimos las fechas
fecha_inicio = '25/09/2023'
dias_vacaciones = 30
dias_no_laborables = ['12/10/2023', '30/10/2023', '06/11/2023']

# Calculamos la fecha de regreso
fecha_regreso = calcular_fecha_regreso(fecha_inicio, dias_vacaciones, dias_no_laborables)

print(f'Debes regresar a trabajar el {fecha_regreso}')
