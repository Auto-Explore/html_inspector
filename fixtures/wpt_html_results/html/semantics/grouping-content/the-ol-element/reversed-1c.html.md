# html/semantics/grouping-content/the-ol-element/reversed-1c.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/reversed-1c.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Reversed numbering should update on dynamic addition of child nodes</title>
<link rel=match href="reversed-1-ref.html">
<link rel=help href="https://html.spec.whatwg.org/#attr-ol-reversed">
<ol id="x" reversed>
  <li>Three</li>
  <li>Two</li>
</ol>
<script>
  var l = document.getElementById("x");
  var w = l.offsetWidth;
  var li = document.createElement("li");
  li.textContent = "One"
  l.appendChild(li);
</script>
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
  "source_name": "html/semantics/grouping-content/the-ol-element/reversed-1c.html"
}
```
