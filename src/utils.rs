// <!doctype html>
//     <html lang="en">
//     <head>
//         <meta charset="UTF-8"/>
//         <title>{{ TITLE }}</title>
//         <link href="/styles/style.css" rel="stylesheet"/>
//     </head>
//     <body>
//          <h1>{{ TITLE }}</h1>
//
//     </body>
// </html>

use markdown;

pub fn convert_to_html(markdown: String) -> String {
    markdown::to_html(&markdown)
}
