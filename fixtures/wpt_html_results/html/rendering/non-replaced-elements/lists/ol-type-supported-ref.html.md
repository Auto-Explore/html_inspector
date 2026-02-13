# html/rendering/non-replaced-elements/lists/ol-type-supported-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ol-type-supported-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>ol@type: supported types</title>
<style>
.decimal {
  list-style-type: decimal;
}
.lower-alpha {
  list-style-type: lower-alpha;
}
.upper-alpha {
  list-style-type: upper-alpha;
}
.lower-roman {
  list-style-type: lower-roman;
}
.upper-roman {
  list-style-type: upper-roman;
}
</style>
<ol class=decimal><li>1<li>2</ol>
<ol class=lower-alpha><li>a<li>b</ol>
<ol class=upper-alpha><li>A<li>B</ol>
<ol class=lower-roman><li>i<li>ii</ol>
<ol class=upper-roman><li>I<li>II</ol>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/lists/ol-type-supported-ref.html"
}
```
