# html/rendering/non-replaced-elements/the-page/crashtests/body-huge-attr-value-crash.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/crashtests/body-huge-attr-value-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml" xlink="http://www.w3.org/1999/xlink">
<meta charset="utf-8">
<link rel="author" href="mailto:0xdevssh@gmail.com">
<link rel="help" href="https://crbug.com/1238543">
<meta name="assert" content="The renderer should not crash.">
    <body topmargin="62099815446794541154677680790275022478020296315411546776807902750224780254115467768079027502247802349339973475906291022478020296313493399734759062916145353182409398397817693775043304346030250203150386848140862367251147544337900224780202963134933997347590629161453531824093983978176937750433043460302502031503868481408623672511475443379061453531824093983978176937750433043460302502031503868481408623672511475443379012223934551240400478945189095326288308770771506289350685029953926171692408537507190372257890049958209064274944598761777623561353561543911257846386780780067311874929239209275379981860757">

    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 80,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-page/crashtests/body-huge-attr-value-crash.html"
}
```
