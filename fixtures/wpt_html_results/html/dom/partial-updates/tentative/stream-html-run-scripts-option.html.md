# html/dom/partial-updates/tentative/stream-html-run-scripts-option.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/stream-html-run-scripts-option.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>streamAppendHTMLUnsafe with scripts</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="placeholder"></div>
<script>

for (const method of ["streamAppendHTMLUnsafe", "streamHTMLUnsafe"]) {
    promise_test(async t => {
        const placeholder = document.getElementById("placeholder");
        const writable = placeholder[method]({runScripts: true});
        const writer = writable.getWriter();
        t.add_cleanup(() => { window.did_run = false });
        await writer.write("<script>window.did_run = true;</" + "script>");
        await writer.close();
        assert_true(window.did_run);
    }, `element.${method} with runScripts: true`);

    promise_test(async t => {
        const placeholder = document.getElementById("placeholder");
        const writable = placeholder[method]({runScripts: false});
        const writer = writable.getWriter();
        t.add_cleanup(() => { window.did_run = false });
        await writer.write("<script>window.did_run = true;</" + "script>");
        await writer.close();
        assert_false(window.did_run);
    }, `element.${method} with runScripts: false`);


    promise_test(async t => {
        const placeholder = document.getElementById("placeholder");
        const element = document.createElement("div");
        placeholder.append(element);
        t.add_cleanup(() => element.remove());
        const shadowRoot = element.attachShadow({mode: "open"});
        const writer = shadowRoot[method]({runScripts: false}).getWriter();
        window.did_run = false;
        await writer.write("<script>window.did_run = true;<" + "/script>");
        await writer.close();
        assert_false(window.did_run);
    }, `shadowRoot.${method} with runScripts: false`);

    promise_test(async t => {
        const placeholder = document.getElementById("placeholder");
        const element = document.createElement("div");
        placeholder.append(element);
        t.add_cleanup(() => element.remove());
        const shadowRoot = element.attachShadow({mode: "open"});
        const writer = shadowRoot[method]({runScripts: true}).getWriter();
        window.did_run = false;
        await writer.write("<script>window.did_run = true;<" + "/script>");
        await writer.close();
        assert_true(window.did_run);
    }, `shadowRoot.${method} with runScripts: true`);
}
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
  "source_name": "html/dom/partial-updates/tentative/stream-html-run-scripts-option.html"
}
```
