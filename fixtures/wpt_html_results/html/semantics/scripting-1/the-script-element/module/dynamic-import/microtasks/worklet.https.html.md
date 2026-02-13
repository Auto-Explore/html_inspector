# html/semantics/scripting-1/the-script-element/module/dynamic-import/microtasks/worklet.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/microtasks/worklet.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel="help" href="https://html.spec.whatwg.org/#hostimportmoduledynamically(referencingscriptormodule,-specifier,-promisecapability)">
<link rel="match" href="worklet-ref.https.html">
<style>
#output {
    width: 100px;
    height: 100px;
    background-image: paint(rects);
    background-color: blue;
}
</style>
<script src="/common/reftest-wait.js"></script>
<script src="/common/worklet-reftest.js"></script>
<body>
<div id="output"></div>

<script id="code" type="text/worklet">
registerPaint('rects', class {
  async paint(ctx, geom) {
    ctx.fillStyle = 'red';

    const getCount = ticker(1000);

    try {
        // Use Date.now() to ensure that the module is not in the module map
        await import("./empty-module.js?" + Date.now());
    } catch (e) {
      if (getCount() < 1000) {
        ctx.fillStyle = 'green';
      }
    }
    ctx.fillRect(0, 0, geom.width, geom.height);
  }
});
</script>

<script>
"use strict";
CSS.paintWorklet.addModule("./ticker.js").then(() =>
  importWorkletAndTerminateTestAfterAsyncPaint(CSS.paintWorklet, document.getElementById('code').textContent)
);
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/microtasks/worklet.https.html"
}
```
