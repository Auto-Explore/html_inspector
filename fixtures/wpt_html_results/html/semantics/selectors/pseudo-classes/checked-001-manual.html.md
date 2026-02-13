# html/semantics/selectors/pseudo-classes/checked-001-manual.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/checked-001-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.0//EN">
<html>
 <head>
  <title>CSS Selectors (:checked)</title>
  <link rel="author" title="Ian Hickson" href="mailto:ian@hixie.ch"/>
  <link rel="alternate" href="http://www.hixie.ch/tests/adhoc/css/selectors/checked/001.html"/>
    <style type="text/css">
   :checked, :checked + span { border: solid blue; color: blue; background: navy; }
  </style>
 </head>
 <body>
  <p>Anything that is checked below should be blue.</p>
  <p><input checked type="checkbox"> <span>X</span></p>
  <p><input checked type="radio" name="x"> <span>X</span> <input checked type="radio" name="x"> <span>X</span></p>
  <p><select><option selected>X</option></select></p>
  <p><select size="2"><option selected>X</option></select></p>
 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.doctype.not_html5",
      "message": "Obsolete doctype. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 300,
        "byte_start": 277,
        "col": 5,
        "line": 7
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/selectors/pseudo-classes/checked-001-manual.html"
}
```
