# html/semantics/interactive-elements/the-dialog-element/dialog-cancel-events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Test cancel event is fired when the dialog is closed by user close requests</title>
<link rel="help" href="https://bugs.webkit.org/show_bug.cgi?id=227534">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1322947">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/common/top-layer.js"></script>
<script src="/close-watcher/resources/helpers.js"></script>

<dialog>
  <p>Hello World</p>
</dialog>

<script type="module">
setup({ single_test: true });

const dialog = document.querySelector("dialog");
const events = [];

dialog.addEventListener("cancel", event => {
  assert_true(event.cancelable, "cancel event should be cancelable");
  assert_array_equals(events, []);

  events.push("addEventListener cancel");
});

assert_equals(dialog.oncancel, null);
dialog.oncancel = () => {
  assert_array_equals(events, ["addEventListener cancel"]);

  events.push("oncancel");
};

dialog.addEventListener("close", () => {
  assert_array_equals(events, ["addEventListener cancel", "oncancel"]);

  events.push("addEventListener close");
});

assert_equals(dialog.onclose, null);
dialog.onclose = () => {
  assert_array_equals(events, ["addEventListener cancel", "oncancel", "addEventListener close"]);

  done();
};

dialog.showModal();
await blessTopLayer(dialog);
await sendCloseRequest();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-cancel-events.html"
}
```
