# html/rendering/non-replaced-elements/tables/table-cell-nowrap-with-fixed-width.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-cell-nowrap-with-fixed-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/#tables-2">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=821915">
<link rel="match" href="table-cell-nowrap-with-fixed-width-ref.html">
<title>A td element with the nowrap attribute should unconditionally apply white-space:nowrap</title>
<style>
table { border-spacing: 0; }
td { width: 10px; padding: 0; }
div { display: inline-block; background: green; width: 50px; height: 100px; }
</style>
<table>
  <td nowrap>
    <div></div><div></div>
  </td>
</table>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-cell-nowrap-with-fixed-width.html"
}
```
