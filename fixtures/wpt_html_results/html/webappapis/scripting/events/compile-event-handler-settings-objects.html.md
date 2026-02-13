# html/webappapis/scripting/events/compile-event-handler-settings-objects.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/compile-event-handler-settings-objects.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Entry and incumbent settings objects when compiling an inline event handler</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script>
"use strict";
window.name = "parent frame";

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "resources/compiled-event-handler-settings-objects-support.html";
  iframe.onload = t.step_func(() => {
    const button = iframe.contentDocument.querySelector("button");
    const compiled = button.onclick;

    assert_equals(compiled.constructor, iframe.contentWindow.Function, "The constructor must be from the iframe");
    assert_not_equals(compiled.constructor, Function, "The constructor must not be from this page");

    t.done();
  });

  document.body.appendChild(iframe);

}, "The Function instance must be created in the Realm of the node document");

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "resources/compiled-event-handler-settings-objects-support.html";
  iframe.onload = t.step_func(() => {
    const button = iframe.contentDocument.querySelector("button");

    window.onWindowLoaded = t.step_func_done(url => {
      const pathname = new URL(url).pathname;
      assert_equals(pathname,
        "/html/webappapis/scripting/events/resources/open-window.html");
      // This tests that the entry settings object used to resolve URLs in that window.open() was the same as that
      // of the node document (i.e. the iframe document), not e.g. this window.
    });

    button.click();
  });

  document.body.appendChild(iframe);

}, "The entry settings object while executing the compiled callback via Web IDL's invoke must be that " +
   "of the node document");

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "resources/compiled-event-handler-settings-objects-support.html";
  iframe.onload = t.step_func(() => {
    window.onmessage = t.step_func_done(event => {
      assert_equals(event.data, "PASS");
      assert_equals(event.source.name, "iframe");
      assert_equals(event.source, iframe.contentWindow, "The source must be the iframe");
    });

    iframe.src = "about:blank";
  });

  document.body.appendChild(iframe);

}, "The incumbent settings object while executing the compiled callback via Web IDL's invoke must be that " +
   "of the node document");
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
  "source_name": "html/webappapis/scripting/events/compile-event-handler-settings-objects.html"
}
```
