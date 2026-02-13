# html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Rendering of tall cross-domain iframe element in a scrolled window</title>
<link rel="match" href="tall-cross-domain-iframe-in-scrolled.sub-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#embedded-content-rendering-rules">
<meta name="assert" content="Checks that tall cross-domain iframe in a scrolled window is correctly rendered">
<iframe id=myframe style="width: 300px; height: 1000px"
    src="http://{{domains[www1]}}:{{ports[http][0]}}/html/rendering/replaced-elements/embedded-content/resources/tall.html"></iframe>
<div style="height: 2000px"></div>
<script>
  window.scrollTo(0, 700);
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
      "message": "Bad value “http://{{domains[www1]}}:{{ports[http][0]}}/html/rendering/replaced-elements/embedded-content/resources/tall.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 605,
        "byte_start": 425,
        "col": 1,
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
  "source_name": "html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub.html"
}
```
