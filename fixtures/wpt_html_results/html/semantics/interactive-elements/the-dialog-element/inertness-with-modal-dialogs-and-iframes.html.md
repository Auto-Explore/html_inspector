# html/semantics/interactive-elements/the-dialog-element/inertness-with-modal-dialogs-and-iframes.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inertness-with-modal-dialogs-and-iframes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Inertness with modal dialogs and iframes</title>
<link rel="author" title="Oriol Brufau" href="mailto:obrufau@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#inert">
<meta name="assert" content="Checks that a modal dialog marks outer nodes as inert,
  but only in its document, not in the parent browsing context,
  nor in nested browsing contexts.">
<div id="log"></div>
<div id="wrapper">
  (main document: outer text)
  <iframe id="outerIframe" srcdoc="
    <div id='wrapper'>
      (outer iframe: outer text)
      <dialog id='dialog' style='display: block'>
        (outer iframe: dialog)
      </dialog>
    </div>
  "></iframe>
  <dialog id="dialog" style="display: block">
    (main document: dialog)
    <iframe id="innerIframe" srcdoc="
      <div id='wrapper'>
        (inner iframe: outer text)
        <dialog id='dialog' style='display: block'>
          (inner iframe: dialog)
        </dialog>
      </div>
    "></iframe>
  </dialog>
</div>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
const innerIframeWindow = innerIframe.contentWindow;
const outerIframeWindow = outerIframe.contentWindow;
promise_setup(async () => {
  for (let global of [innerIframeWindow, outerIframeWindow]) {
    if (global.location.href === "about:blank" ||
        global.document.readyState !== "complete") {
      await new Promise(resolve => {
        global.frameElement.addEventListener("load", resolve, {once: true});
      });
    }
  }
});
add_completion_callback(() => {
  for (let global of [window, innerIframeWindow, outerIframeWindow]) {
    global.getSelection().removeAllRanges();
  }
});

function checkSelection(global, expectedText) {
  const selection = global.getSelection();
  selection.selectAllChildren(global.wrapper);

  // Remove whitespace between parentheses since it varies among browsers,
  // but that's not relevant to this test.
  const actualText = selection.toString().replace(/\)\s*\(/g, ")(").trim();
  assert_equals(actualText, expectedText);
}

function showModals(test, globals) {
  for (let global of globals) {
    global.dialog.showModal();
    test.add_cleanup(() => { global.dialog.close(); });
  }
}

promise_test(async function() {
  checkSelection(window, "(main document: outer text)(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: outer text)(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: outer text)(outer iframe: dialog)");
}, "Initially, no node is inert");

promise_test(async function() {
  showModals(this, [outerIframeWindow]);

  checkSelection(window, "(main document: outer text)(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: outer text)(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: dialog)");
}, "Modal dialog in the outer iframe marks outer nodes in that iframe as inert.");

promise_test(async function() {
  showModals(this, [innerIframeWindow]);

  checkSelection(window, "(main document: outer text)(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: outer text)(outer iframe: dialog)");
}, "Modal dialog in the inner iframe marks outer nodes in that iframe as inert.");

promise_test(async function() {
  showModals(this, [innerIframeWindow, outerIframeWindow]);

  checkSelection(window, "(main document: outer text)(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: dialog)");
}, "Modal dialogs in both iframes mark outer nodes in these iframes as inert.");

promise_test(async function() {
  showModals(this, [window]);

  checkSelection(window, "(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: outer text)(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: outer text)(outer iframe: dialog)");
}, "Modal dialog in the main document marks outer nodes as inert. Contents of the outer iframe aren't marked as inert.");

promise_test(async function() {
  showModals(this, [innerIframeWindow, window]);

  checkSelection(window, "(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: outer text)(outer iframe: dialog)");
}, "Modal dialogs in the main document and inner iframe mark outer nodes as inert. Contents of the outer iframe aren't marked as inert.");

promise_test(async function() {
  showModals(this, [outerIframeWindow, window]);

  checkSelection(window, "(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: outer text)(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: dialog)");
}, "Modal dialogs in the main document and outer iframe mark outer nodes as inert. Contents of the outer iframe aren't marked as inert.");

promise_test(async function() {
  showModals(this, [innerIframeWindow, outerIframeWindow, window]);

  checkSelection(window, "(main document: dialog)");
  checkSelection(innerIframeWindow, "(inner iframe: dialog)");
  checkSelection(outerIframeWindow, "(outer iframe: dialog)");
}, "Modal dialogs in the main document and both iframes mark outer nodes as inert. Contents of the outer iframe aren't marked as inert.");
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inertness-with-modal-dialogs-and-iframes.html"
}
```
