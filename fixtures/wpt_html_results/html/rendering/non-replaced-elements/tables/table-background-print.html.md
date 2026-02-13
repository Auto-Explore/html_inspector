# html/rendering/non-replaced-elements/tables/table-background-print.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-background-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<link rel="match" href="table-background-print-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="help" href="https://drafts.csswg.org/css-break/">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=816498">
<style>
  :root {
    print-color-adjust: exact;
  }
</style>
<body>
<table background="resources/aqua-yellow-32x32.png">
  <thead background="resources/blue-16x20-green-16x20.png">
    <tr>
      <td>
        Foo
      </td>
      <td background="resources/yellow-32x32.png">
        Bar
      </td>
    </tr>
  </thead>
  <tbody background="resources/red-32x32.png">
    <tr>
      <th background="resources/fuchsia-32x32.png">
        Foo
      </th>
      <th>
        Bar
      </th>
    </tr>
    <tr background="resources/fuchsia-32x32.png">
      <td>
        Foo
      </td>
      <td background="resources/yellow-32x32.png">
        Bar
      </td>
    </tr>
  </tbody>
  <tfoot background="resources/yellow-32x32.png">
    <tr>
      <td>
        Baz
      </td>
    </tr>
  </tfoot>
</table>
</body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-background-print.html"
}
```
