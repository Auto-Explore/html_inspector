# html/semantics/document-metadata/the-style-element/update-style-block-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/update-style-block-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#update-a-style-block">
<link rel="match" href="update-style-block-ascii-case-insensitive-ref.html">
<meta name="assert" content="style@type values are ASCII case-insensitive">
<style>
p:after { font-weight: bold; }
p:after { content: "FAIL"; color: red; }
#c:after { content: "PASS"; color: green; }
</style>
<style type="text/css">#a:after { content: "PASS"; color: green; }</style>
<style type="TeXt/CsS">#b:after { content: "PASS"; color: green; }</style>
<style type="text/cſs">#c:after { content: "FAIL"; color: red; }</style>
<p id="a">text/css treated as CSS?
<p id="b">TeXt/CsS treated as CSS?
<p id="c">text/cſs ignored?
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 424,
        "byte_start": 401,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 499,
        "byte_start": 476,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.text_css_only",
      "message": "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 575,
        "byte_start": 551,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/document-metadata/the-style-element/update-style-block-ascii-case-insensitive.html"
}
```
