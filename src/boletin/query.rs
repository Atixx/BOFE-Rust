use chrono::{NaiveDate, Utc};
use serde::Serialize;

#[derive(Debug)]
pub struct QueryInfo {
    search_string: String,
    from_date: NaiveDate,
    to_date: NaiveDate,
}

impl Default for QueryInfo {
    fn default() -> QueryInfo {
        QueryInfo {
            search_string: String::from(""),
            from_date: Utc::today().naive_utc(),
            to_date: Utc::today().naive_utc(),
        }
    }
}

impl QueryInfo {
    pub fn new(search_string: &str, from_date: NaiveDate, to_date: NaiveDate) -> QueryInfo {
        QueryInfo {
            search_string: String::from(search_string),
            from_date,
            to_date,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoletinQuery {
    busqueda_rubro: bool,
    hay_mas_resultados_busqueda: bool,
    ejecutando_llamada_asincronica_busqueda: bool,
    ultima_seccion: String,
    filtro_por_rubros_seccion: bool,
    filtro_por_rubro_busqueda: bool,
    filtro_por_seccion_busqueda: bool,
    busqueda_original: bool,
    ordenamiento_segunda: bool,
    secciones_originales: Vec<u32>,
    ultimo_item_externo: Option<u32>,
    ultimo_item_interno: Option<u32>,
    texto: String,
    rubros: Vec<u32>,
    nro_norma: String,
    anio_norma: String,
    denominacion: String,
    tipo_contratacion: String,
    anio_contratacion: String,
    nro_contratacion: String,
    fecha_desde: String,
    fecha_hasta: String,
    todas_las_palabras: bool,
    comienza_denominacion: bool,
    seccion: Vec<u32>,
    tipo_busqueda: String,
    numero_pagina: u32,
    ultimo_rubro: String,
}

trait FormatDate {
    fn format_date(date: NaiveDate) -> String;
}

impl FormatDate for BoletinQuery {
    fn format_date(date: NaiveDate) -> String {
        date.format("%d/%m/%Y").to_string()
    }
}

impl Default for BoletinQuery {
    fn default() -> BoletinQuery {
        BoletinQuery {
            busqueda_rubro: false,
            hay_mas_resultados_busqueda: true,
            ejecutando_llamada_asincronica_busqueda: false,
            ultima_seccion: String::from(""),
            filtro_por_rubros_seccion: false,
            filtro_por_rubro_busqueda: false,
            filtro_por_seccion_busqueda: false,
            busqueda_original: true,
            ordenamiento_segunda: false,
            secciones_originales: vec![1, 2, 3],
            ultimo_item_externo: None,
            ultimo_item_interno: None,
            texto: String::from(""),
            rubros: vec![],
            nro_norma: String::from(""),
            anio_norma: String::from(""),
            denominacion: String::from(""),
            tipo_contratacion: String::from(""),
            anio_contratacion: String::from(""),
            nro_contratacion: String::from(""),
            fecha_desde: BoletinQuery::format_date(Utc::today().naive_utc()),
            fecha_hasta: BoletinQuery::format_date(Utc::today().naive_utc()),
            todas_las_palabras: true,
            comienza_denominacion: true,
            seccion: vec![1, 2, 3],
            tipo_busqueda: String::from("Avanzada"),
            numero_pagina: 1,
            ultimo_rubro: String::from(""),
        }
    }
}

impl BoletinQuery {
    pub fn new(info: &QueryInfo) -> BoletinQuery {
        BoletinQuery {
            texto: info.search_string.replace(" ", "+"),
            fecha_desde: BoletinQuery::format_date(info.from_date),
            fecha_hasta: BoletinQuery::format_date(info.to_date),
            ..Default::default()
        }
    }

    /// Builds query from BoletinQuery
    ///
    /// Uses BoletinQuery from constructed QueryInfo parameters
    pub fn build_query(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = String::from("params=");
        query.push_str(&serde_json::to_string(self)?);
        query.push_str("&array_volver=[]");
        query.retain(|c| !c.is_whitespace());

        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn builds_query_correctly() {
        let info = QueryInfo {
            search_string: String::from("Policia Seguridad Aeroportuaria"),
            from_date: NaiveDate::from_ymd(2021, 8, 3),
            to_date: NaiveDate::from_ymd(2021, 8, 5),
        };

        let query = BoletinQuery::new(&info);

        assert_eq!(
            query.build_query().unwrap(),
            r#"params={"busquedaRubro":false,"hayMasResultadosBusqueda":true,"ejecutandoLlamadaAsincronicaBusqueda":false,"ultimaSeccion":"","filtroPorRubrosSeccion":false,"filtroPorRubroBusqueda":false,"filtroPorSeccionBusqueda":false,"busquedaOriginal":true,"ordenamientoSegunda":false,"seccionesOriginales":[1,2,3],"ultimoItemExterno":null,"ultimoItemInterno":null,"texto":"Policia+Seguridad+Aeroportuaria","rubros":[],"nroNorma":"","anioNorma":"","denominacion":"","tipoContratacion":"","anioContratacion":"","nroContratacion":"","fechaDesde":"03/08/2021","fechaHasta":"05/08/2021","todasLasPalabras":true,"comienzaDenominacion":true,"seccion":[1,2,3],"tipoBusqueda":"Avanzada","numeroPagina":1,"ultimoRubro":""}&array_volver=[]"#
        )
    }
}
