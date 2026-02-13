# html/rendering/non-replaced-elements/lists/ol-start-display-contents.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ol-start-display-contents.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>display: contents; on &lt;ol start></title>
<meta rel=match href=ol-display-contents-ref.html>
<ol start=5 style="display: contents">
  <li style="margin-left: 40px">The list item marker on this line should be "1."</li>
  <li style="margin-left: 40px">The list item marker on this line should be "2."</li>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 138,
        "byte_start": 88,
        "col": 1,
        "line": 4
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
  "source_name": "html/rendering/non-replaced-elements/lists/ol-start-display-contents.html"
}
```
