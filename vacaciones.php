<?php

function calcularFechaRegreso($fechaInicio, $diasVacaciones, $diasNoLaborables) {
    $fechaActual = new DateTime($fechaInicio);
    $diasFaltantes = 0;

    while ($diasFaltantes < $diasVacaciones) {
        $fechaActual->add(new DateInterval('P1D'));

        $fechaActualFormat = $fechaActual->format('d-m-Y');
        $diaSemana = $fechaActual->format('N');

        if (! in_array($fechaActualFormat, $diasNoLaborables) && $diaSemana < 6) {
            $diasFaltantes++;
        }
    }

    return $fechaActual->format('d-m-Y');
}

$fechaInicio = '25-09-2023';
$diasVacaciones = 30;
$diasNoLaborables = ['12-10-2023', '30-10-2023', '06-11-2023'];

$fechaRegreso = calcularFechaRegreso($fechaInicio, $diasVacaciones, $diasNoLaborables);

echo "Debes regresar a trabajar el $fechaRegreso\n";
