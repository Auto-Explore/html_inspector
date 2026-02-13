# html/rendering/bidi-rendering/slot-no-isolate-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bidi-rendering/slot-no-isolate-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML Rendering: slot element has unicode-bidi: isolate</title>
<link rel="author" title="L. David Baron" href="https://dbaron.org/">
<link rel="author" title="Google" href="http://www.google.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#bidi-rendering">
<link rel="help" href="https://github.com/whatwg/html/pull/9880">
<link rel="match" href="slot-no-isolate-001-ref.html">

<div>&#x5D0;-<span id="e">&#x5D1;</span></div>

<div id="v"></div>

<script>

let e = document.getElementById("e");
let root = e.attachShadow({mode: "open"});
// use display:inline to override default display:contents
root.innerHTML = "<slot style='display:inline'>\u05D2</slot>";
let val = getComputedStyle(root.querySelector("slot")).unicodeBidi;
document.getElementById("v").innerText = val;

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
  "source_name": "html/rendering/bidi-rendering/slot-no-isolate-001.html"
}
```
