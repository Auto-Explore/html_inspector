# html/rendering/replaced-elements/embedded-content/cross-domain-iframe-in-multicol.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/cross-domain-iframe-in-multicol.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Rendering of cross-domain iframe element in multicol</title>
<link rel="match" href="cross-domain-iframe-in-multicol.sub-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules">
<meta name="assert" content="Checks that cross-domain iframe in multicol is correctly rendered">
<div style="columns: 2; height: 300px">
  <div style="height: 100px"></div>
  <iframe id=myframe src="http://{{domains[www1]}}:{{ports[http][0]}}/images/green.png"></iframe>
  <div style="height: 100px"></div>
</div>
<script>
  myframe.onload = () => {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        document.documentElement.classList.remove('reftest-wait');
      });
    });
  }
</script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{domains[www1]}}:{{ports[http][0]}}/images/green.png” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 556,
        "byte_start": 470,
        "col": 3,
        "line": 9
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
  "source_name": "html/rendering/replaced-elements/embedded-content/cross-domain-iframe-in-multicol.sub.html"
}
```
