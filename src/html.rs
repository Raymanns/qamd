// use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use horrorshow::{Render, RenderBox};

use report::{Locator, Report};

use std::collections::HashSet;

pub fn to_html(report: &Report) -> String {
    let javascript = r#"
'use strict';

$(function() {

  $('tr.table-danger').click(function(obj) {
    var name = obj.currentTarget.children[0].innerText

    $('h2#selected-check').first().removeClass('d-none');
    $('h2#selected-check')[0].innerText = name;

    var selector = 'table#' + name.toLowerCase().replace(/ /g, '_');

    // hide all the tables
    $('table.table.table-striped').each(function(index, elem) {
      $('table#' + elem.id).addClass('d-none');
    });

    // show the selected table
    $(selector).first().removeClass('d-none');
  });
});
"#;

    format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title : &report.metadata.file_name;
                    meta(charset="UTF-8");
                    link(rel="stylesheet",
                         href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/css/bootstrap.min.css",
                         integrity="sha384-WskhaSGFgHYWDcbwN70/dfYBj47jz9qbsMId/iRN3ewGhXQFZCSftd1LZCfmhktB",
                         crossorigin="anonymous");
                }

                body {
                    div(class="container") {
                        div(id="title", class="row") {
                            h1(id="file-name") : &report.metadata.file_name;
                        }

                        div(class="row metadata") {
                            strong : format!("Raw Case Count: {}",
                                      report.metadata.raw_case_count);
                        }

                        div(class="row metadata") {
                            @ if let Some(case_count) = report.metadata.case_count {
                                strong : format!("Aggregated Case Count: {}",
                                          case_count);
                            }
                        }

                        div(class="row metadata") {
                            strong : format!("Total Variables: {}",
                                      report.metadata.variable_count);
                        }

                        div(class="row metadata") {
                            strong : format!("Created At: {}",
                                      report.metadata.creation_time);
                        }

                        div(class="row metadata") {
                            strong : format!("Last modified at: {}",
                                             report.metadata.modified_time);
                        }

                        div(class="row metadata") {
                            strong : format!("File Label: {}",
                                      &report.metadata.file_label);
                        }

                        div(class="row metadata") {
                            strong : format!("File Format Version: {}",
                                      report.metadata.file_format_version);
                        }

                        div(class="row metadata") {
                            @ if let Some(ref file_encoding) = &report.metadata.file_encoding {
                                strong : format!("File Encoding: {}", file_encoding);
                            }
                        }

                        div(class="row metadata") {
                            strong : format!("Compression type: {}",
                                      &report.metadata.compression);
                        }

                        br;

                        div(id="report", class="row") {
                            table(class="table table-bordered") {
                                tr {
                                    th(scope="col") : "Name";
                                    th(scope="col") : "Status";
                                    th(scope="col") : "Description";
                                }

                                @ for (name, status) in report.summary.iter() {
                                    @ if status.fail > 0 {
                                        tr(class="table-danger") {
                                            td(scope="row") : format!("{}", name);
                                            td : format!("failed ({})", status.fail);
                                            td : &status.desc;
                                        }
                                    } else {
                                        tr(class="table-success") {
                                            td(scope="row") : format!("{}", name);
                                            td : "passed";
                                            td : &status.desc;
                                        }
                                    }
                                }
                            }
                        }

                        br;

                        div(class="row") {
                            h2(id="selected-check", class="d-none") : "hidden";
                        }

                        @ for (name, status) in report.summary.iter() {
                            @ if let Some(ref locators) = status.locator {
                                : locators_table(format!("{}", name),
                                                 locators.clone());
                            }
                        }
                    }

                    script(src="https://code.jquery.com/jquery-3.3.1.slim.min.js",
                           integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo",
                           crossorigin="anonymous") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.3/umd/popper.min.js",
                           integrity="sha384-ZMP7rVo3mIykV+2+9J3UJ46jBk0WLaUAdn689aCwoqbBJiSnjAK/l8WvCWPIPm49",
                           crossorigin="anonymous") {}
                    script(src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.1/js/bootstrap.min.js",
                           integrity="sha384-smHYKdLADwkXOn1EmN1qk/HfnUcbVRZyYmZ4qpPea6sjB/pTJ0euyQp0Mk8ck+5T",
                           crossorigin="anonymous") {}
                    script(type="text/javascript") {
                         : javascript;
                    }
                }
            }
        }
    )
}

fn locators_table<'a>(name: String, locators: HashSet<Locator>) -> Box<RenderBox> {
    box_html! {
        div(class="row") {
            table(id=name.to_lowercase().replace(" ", "_"),
                  class="table table-striped table-bordered d-none") {
                tr {
                    th(scope="col") : "# (limited to 1000)";
                    th(scope="col") : "Variable (Index)";
                    th(scope="col") : "Row Index";
                    th(scope="col") : "Reason";
                }

                @ for (i, locator) in locators.iter().take(1000).enumerate() {
                    tr(class="locator") {
                        td(scope="row") : i + 1;
                        td : format!("{} ({})",
                                     locator.variable_name,
                                     locator.variable_index);

                        : value_if_positive(locator.value_index, "-");
                        td : value_if_present(&locator.reason, "-".to_string());
                    }
                }
            }
        }
    }
}

fn value_if_positive(value: i32, default: &'static str) -> Box<Render> {
    box_html! {
        @ if value < 0 {
            td : default;
        } else {
            td : value;
        }
    }
}

fn value_if_present(value: &Option<String>, default: String) -> String {
    match value {
        Some(v) => return v.to_string(),
        None => return default,
    }
}

