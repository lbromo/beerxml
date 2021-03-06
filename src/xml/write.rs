// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::Write;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

use data::*;
use error::*;

fn indent<T>(writer: &mut T, offset: usize) -> Result<()>
    where T: Write
{
    for _ in 0..offset {
        writer.write_all(b"  ")?;
    }
    Ok(())
}

fn write_tag<T, U>(writer: &mut T, offset: usize, tag: &'static str, value: &U) -> Result<()>
    where T: Write,
          U: Display
{
    indent(writer, offset + 1)?;
    write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    Ok(())
}

fn write_bool<T>(writer: &mut T, offset: usize, tag: &'static str, value: bool) -> Result<()>
    where T: Write
{
    if value {
        indent(writer, offset + 1)?;
        write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    }
    Ok(())
}



fn write_opt<T, U>(writer: &mut T,
                   offset: usize,
                   tag: &'static str,
                   value: &Option<U>)
                   -> Result<()>
    where T: Write,
          U: Display
{
    if let Some(ref value) = *value {
        indent(writer, offset + 1)?;
        write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    }
    Ok(())
}

fn write_block<F, T>(writer: &mut T, offset: usize, tag: &'static str, containing: F) -> Result<()>
    where T: Write,
          F: Fn(&mut T, usize) -> Result<()>
{
    indent(writer, offset)?;
    write!(writer, "<{}>\n", tag)?;
    containing(writer, offset + 1)?;
    indent(writer, offset)?;
    write!(writer, "</{}>\n", tag)?;
    Ok(())
}

fn write_fermentable<T>(writer: &mut T, f: &Fermentable, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "FERMENTABLE", |writer, offset| {
        write_tag(writer, offset, "NAME", &f.name)?;
        write_tag(writer, offset, "VERSION", &f.version)?;
        write_tag(writer, offset, "TYPE", &f.type_.to_string())?;
        write_tag(writer, offset, "AMOUNT", &f.amount)?;
        write_tag(writer, offset, "YIELD", &f.yield_)?;
        write_tag(writer, offset, "COLOR", &f.color)?;
        write_bool(writer, offset, "ADD_AFTER_BOIL", f.add_after_boil)?;
        write_opt(writer, offset, "ORIGIN", &f.origin)?;
        write_opt(writer, offset, "SUPPLIER", &f.supplier)?;
        write_opt(writer, offset, "NOTES", &f.notes)?;
        write_opt(writer, offset, "COARSE_FINE_DIFF", &f.coarse_fine_diff)?;
        write_opt(writer, offset, "MOISTURE", &f.moisture)?;
        write_opt(writer, offset, "DIASTATIC_POWER", &f.diastatic_power)?;
        write_opt(writer, offset, "PROTEIN", &f.protein)?;
        write_opt(writer, offset, "MAX_IN_BATCH", &f.max_in_batch)?;
        write_bool(writer, offset, "RECOMMEND_MASH", f.recommend_mash)?;
        write_opt(writer, offset, "IBU_GAL_PER_LB", &f.ibu_gal_per_lb)?;
        write_opt(writer, offset, "DISPLAY_AMOUNT", &f.display_amount)?;
        write_opt(writer, offset, "INVENTORY", &f.inventory)?;
        write_opt(writer, offset, "POTENTIAL", &f.potential)?;
        write_opt(writer, offset, "DISPLAY_COLOR", &f.display_color)
    })
}

fn write_hop<T>(writer: &mut T, h: &Hop, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "HOP", |writer, offset| {
        write_tag(writer, offset, "NAME", &h.name)?;
        write_tag(writer, offset, "VERSION", &h.version)?;
        write_tag(writer, offset, "ALPHA", &h.alpha)?;
        write_tag(writer, offset, "AMOUNT", &h.amount)?;
        write_tag(writer, offset, "USE", &h.use_.to_string())?;
        write_tag(writer, offset, "TIME", &h.time)?;
        write_opt(writer, offset, "NOTES", &h.notes)?;
        let type_ = h.type_.as_ref().map(|x| x.to_string());
        write_opt(writer, offset, "TYPE", &type_)?;
        let form = h.form.as_ref().map(|x| x.to_string());
        write_opt(writer, offset, "FORM", &form)?;
        write_opt(writer, offset, "BETA", &h.beta)?;
        write_opt(writer, offset, "HSI", &h.hsi)?;
        write_opt(writer, offset, "ORIGIN", &h.origin)?;
        write_opt(writer, offset, "SUBSTITUTES", &h.substitutes)?;
        write_opt(writer, offset, "HUMULENE", &h.humulene)?;
        write_opt(writer, offset, "CARYOPHYLLENE", &h.caryophyllene)?;
        write_opt(writer, offset, "COHUMULONE", &h.cohumulone)?;
        write_opt(writer, offset, "MYRCENE", &h.myrcene)
    })
}

fn write_yeast<T>(writer: &mut T, y: &Yeast, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "YEAST", |writer, offset| {
        write_tag(writer, offset, "NAME", &y.name)?;
        write_tag(writer, offset, "VERSION", &y.version)?;
        write_tag(writer, offset, "TYPE", &y.type_.to_string())?;
        write_tag(writer, offset, "FORM", &y.form.to_string())?;
        write_tag(writer, offset, "AMOUNT", &y.amount)?;
        write_bool(writer, offset, "AMOUNT_IS_WEIGHT", y.amount_is_weight)?;
        write_opt(writer, offset, "LABORATORY", &y.laboratory)?;
        write_opt(writer, offset, "PRODUCT_ID", &y.product_id)?;
        write_opt(writer, offset, "MIN_TEMPERATURE", &y.min_temperature)?;
        write_opt(writer, offset, "MAX_TEMPERATURE", &y.max_temperature)?;
        let floc = y.flocculation.as_ref().map(|x| x.to_string());
        write_opt(writer, offset, "FLOCCULATION", &floc)?;
        write_opt(writer, offset, "ATTENUATION", &y.attenuation)?;
        write_opt(writer, offset, "NOTES", &y.notes)?;
        write_opt(writer, offset, "BEST_FOR", &y.best_for)?;
        write_opt(writer, offset, "TIMES_CULTURED", &y.times_cultured)?;
        write_opt(writer, offset, "MAX_REUSE", &y.max_reuse)?;
        write_bool(writer, offset, "ADD_TO_SECONDARY", y.add_to_secondary)?;
        write_opt(writer, offset, "DISPLAY_AMOUNT", &y.display_amount)?;
        write_opt(writer, offset, "DISP_MIN_TEMP", &y.display_min_temp)?;
        write_opt(writer, offset, "DISP_MAX_TEMP", &y.display_max_temp)?;
        write_opt(writer, offset, "INVENTORY", &y.inventory)?;
        write_opt(writer, offset, "CULTURE_DATE", &y.culture_date)
    })
}
fn write_misc<T>(writer: &mut T, m: &Misc, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "MISC", |writer, offset| {
        write_tag(writer, offset, "NAME", &m.name)?;
        write_tag(writer, offset, "VERSION", &m.version)?;
        write_tag(writer, offset, "TYPE", &m.type_.to_string())?;
        write_tag(writer, offset, "USE", &m.use_.to_string())?;
        write_tag(writer, offset, "TIME", &m.time)?;
        write_tag(writer, offset, "AMOUNT", &m.amount)?;
        write_bool(writer, offset, "AMOUNT_IS_WEIGHT", m.amount_is_weight)?;
        write_opt(writer, offset, "USE_FOR", &m.use_for)?;
        write_opt(writer, offset, "NOTES", &m.notes)?;
        write_opt(writer, offset, "DISPLAY_TIME", &m.display_time)?;
        write_opt(writer, offset, "DISPLAY_AMOUNT", &m.display_amount)?;
        write_opt(writer, offset, "INVENTORY", &m.inventory)
    })
}
fn write_water<T>(writer: &mut T, w: &Water, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "WATER", |writer, offset| {
        write_tag(writer, offset, "NAME", &w.name)?;
        write_tag(writer, offset, "VERSION", &w.version)?;
        write_tag(writer, offset, "AMOUNT", &w.amount)?;
        write_tag(writer, offset, "CALCIUM", &w.calcium)?;
        write_tag(writer, offset, "BICARBONATE", &w.bicarbonate)?;
        write_tag(writer, offset, "SULFATE", &w.sulfate)?;
        write_tag(writer, offset, "CHLORIDE", &w.chloride)?;
        write_tag(writer, offset, "SODIUM", &w.sodium)?;
        write_tag(writer, offset, "MAGNESIUM", &w.magnesium)?;
        write_opt(writer, offset, "PH", &w.ph)?;
        write_opt(writer, offset, "NOTES", &w.notes)
    })
}

fn write_recipe<T>(_writer: &mut T, _r: &Recipe, _offset: usize) -> Result<()>
    where T: Write
{
    unimplemented!();
}

fn write_style<T>(_writer: &mut T, _s: &Style, _offset: usize) -> Result<()>
    where T: Write
{
    unimplemented!();
}

fn write_mash<T>(_writer: &mut T, _s: &Mash, _offset: usize) -> Result<()>
    where T: Write
{
    unimplemented!();
}

fn write_equipment<T>(_writer: &mut T, _s: &Equipment, _offset: usize) -> Result<()>
    where T: Write
{
    unimplemented!();
}

fn write_map<E, F, T>(writer: &mut T,
                      v: &HashMap<String, E>,
                      offset: usize,
                      name: &'static str,
                      write_element: F)
                      -> Result<()>
    where T: Write,
          F: Fn(&mut T, &E, usize) -> Result<()>
{
    write_block(writer, offset, name, |writer, offset| {
        for f in v.values() {
            write_element(writer, f, offset + 1)?;
        }
        Ok(())
    })
}

/// try to write a `RecordSet` to a `writer`
pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    write!(writer,
           "<!-- written by brewcalc {}: http://brewcalc.org/ -->\n",
           env!("CARGO_PKG_VERSION"))
        ?;
    match *set {
        RecordSet::Empty => Ok(()),
        RecordSet::Equipments(ref v) => write_map(writer, v, 0, "EQUIPMENTS", write_equipment),
        RecordSet::Fermentables(ref v) => write_map(writer, v, 0, "FERMENTABLES", write_fermentable),
        
        RecordSet::Hops(ref v) => write_map(writer, v, 0, "HOPS", write_hop),
        RecordSet::Yeasts(ref v) => write_map(writer, v, 0, "YEASTS", write_yeast),
        RecordSet::Miscs(ref v) => write_map(writer, v, 0, "MISCS", write_misc),
        RecordSet::Recipes(ref v) => write_map(writer, v, 0, "RECIPES", write_recipe),
        RecordSet::Waters(ref v) => write_map(writer, v, 0, "WATERS", write_water),
        RecordSet::Styles(ref v) => write_map(writer, v, 0, "STYLES", write_style),
        RecordSet::Mashs(ref v) => write_map(writer, v, 0, "MASHS", write_mash),
    }
}

/// try to write a `RecordSet` to a file
pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
