# html/cross-origin-embedder-policy/multi-globals/incumbent/incumbent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/multi-globals/incumbent/incumbent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Incumbent page</title>

<iframe src="../current/current.html" id="c"></iframe>

<script>
  const current = document.querySelector("#c").contentWindow;

  window.hello = () => {
    const worker = new current.Worker('worker.js');
    worker.onmessage = e => { parent.postMessage(e.data, '*'); }
  };
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
  "source_name": "html/cross-origin-embedder-policy/multi-globals/incumbent/incumbent.html"
}
```
