# html/dom/partial-updates/tentative/stream-append-has-invalidation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/stream-append-has-invalidation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>HTML partial updates - :has invalidation with streaming</title>
<link rel="help" href="https://github.com/WICG/declarative-partial-updates" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  .container {
    width: 200px;
    height: 100px;
    background: green;
  }

  .container:has(.red) {
    background: red;
  }

  .container:has(.blue) {
    background: blue;
  }
</style>
<div class="container">
  <div contentname="content"><span class="red">Has red</span></div>
</div>

<script>
  const container = document.querySelector(".container");
  async function update(html) {
    const writer = container.streamAppendHTMLUnsafe({runScripts: true}).getWriter();
    await writer.write(`
      <template contentmethod="replace-children">
        <div contentname="content">${html}</div>
      </template>`);
    await writer.close();
  }
  promise_test(async () => {
    for (let i = 0; i < 20; ++i) {
      await update('<span class="blue">Has blue</span>');
      await new Promise((resolve) => requestAnimationFrame(resolve));
      await update("Green (no span)");
      await new Promise((resolve) => requestAnimationFrame(resolve));
      assert_equals(container.textContent.trim(), "Green (no span)");
      assert_equals(
        getComputedStyle(container).backgroundColor,
        "rgb(0, 128, 0)"
      );
    }
  }, "");
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
  "source_name": "html/dom/partial-updates/tentative/stream-append-has-invalidation.html"
}
```
