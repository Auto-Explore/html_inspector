# html/semantics/interactive-elements/the-dialog-element/dialog-toggle-source.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-toggle-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9111">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../../popovers/resources/toggle-event-source-test.js"></script>

<button id=showmodalbutton commandfor=dialog command=show-modal>show modal</button>
<button id=lightdismiss>light dismiss</button>
<dialog id=dialog closedby=any>
  dialog
  <button id=closebutton commandfor=dialog command=close>close</button>
  <button id=requestclosebutton commandfor=dialog command=request-close>request close</button>
</dialog>

<script>
const showmodalbutton = document.getElementById('showmodalbutton');
const dialog = document.getElementById('dialog');
const requestclosebutton = document.getElementById('requestclosebutton');
const lightdismiss = document.getElementById('lightdismiss');

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <dialog> elements: dialog.showModal().',
  target: dialog,
  openFunc: async () => dialog.showModal(),
  closeFunc: async () => dialog.close(),
  openSource: null,
  closeSource: null
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <dialog> elements: command button.',
  target: dialog,
  openFunc: async () => showmodalbutton.click(),
  closeFunc: async () => closebutton.click(),
  openSource: showmodalbutton,
  closeSource: closebutton
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <dialog> elements: open with showModal, close with button.',
  target: dialog,
  openFunc: async () => dialog.showModal(),
  closeFunc: async () => closebutton.click(),
  openSource: null,
  closeSource: closebutton
});

createToggleEventSourceTest({
  description: 'ToggleEvent.soruce on <dialog> elements: open with button, close with dialog.close().',
  target: dialog,
  openFunc: async () => showmodalbutton.click(),
  closeFunc: async () => dialog.close(),
  openSource: showmodalbutton,
  closeSource: null
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <dialog> elements: open with showModal, close with request-close button.',
  target: dialog,
  openFunc: async () => dialog.showModal(),
  closeFunc: async () => requestclosebutton.click(),
  openSource: null,
  closeSource: requestclosebutton
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <dialog> elements: open with button, close with light dismiss.',
  target: dialog,
  openFunc: async () => {
    await test_driver.click(showmodalbutton);
  },
  closeFunc: async () => {
    dialog.addEventListener('cancel', event => event.preventDefault(), {once: true});
    requestclosebutton.click();
    assert_true(dialog.matches('[open]'),
      'cancel preventDefault should have prevented dialog from closing.');
    await (new test_driver.Actions()
      .pointerMove(0, 0, {origin: lightdismiss})
      .pointerDown()
      .pointerUp())
      .send();
  },
  openSource: showmodalbutton,
  closeSource: null
});
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-toggle-source.html"
}
```
