# html/semantics/interactive-elements/the-dialog-element/modal-dialog-ancestor-is-inert.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-ancestor-is-inert.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:falken@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=329407">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<style>
#ancestor {
  position: absolute;
  height: 50px;
  width: 50px;
  top: 200px;
  left: 100px;
  border: 1px solid;
}

dialog {
  height: 50px;
  width: 50px;
  top: 200px;
  left: 200px;
  margin: 0;
}

dialog::backdrop {
  display: none;
}
</style>

<div id="ancestor">
  <dialog></dialog>
</div>

<script>
promise_test(async () => {
  async function clickOn(element) {
    const rect = element.getBoundingClientRect();
    const actions = new test_driver.Actions()
      .pointerMove(rect.left + rect.width / 2, rect.top + rect.height / 2)
      .pointerDown()
      .pointerUp();
    await actions.send();
  }

  const div = document.querySelector('#ancestor');
  const dialog = document.querySelector('dialog');
  dialog.showModal();

  const handledEvent = {};
  document.addEventListener('click', function(event) {
    handledEvent['document'] = true;
  });

  document.body.addEventListener('click', function(event) {
    handledEvent['body'] = true;
    // body should get a event only via bubbling.
    if (event.target != dialog) {
      assert_unreached('body was targeted for an click event');
      div.style.backgroundColor = 'red';
    }
  });

  div.addEventListener('click', function(event) {
    handledEvent['div'] = true;
    // div should get a event only via bubbling.
    if (event.target != dialog) {
      assert_unreached('div was targeted for an click event');
      div.style.backgroundColor = 'red';
    }
  });

  dialog.addEventListener('click', function(event) {
    handledEvent['dialog'] = true;
    dialog.style.backgroundColor = 'green';
    if (event.target != dialog) {
      assert_unreached('dialog was not targeted for a click event');
      dialog.style.backgroundColor = 'red';
    }
  });

  const nodes = [ 'document', 'body', 'div', 'dialog' ];
  nodes.map(function(node) { handledEvent[node] = false; });
  await clickOn(div);
  assert_true(handledEvent.document, 'Clicking on ancestor.');
  assert_false(handledEvent.body, 'Clicking on ancestor.');
  assert_false(handledEvent.dialog, 'Clicking on ancestor.');
  assert_false(handledEvent.div, 'Clicking on ancestor.');
  handledEvent.document = false;

  await clickOn(dialog);
  assert_true(handledEvent.document, 'Clicking on dialog.');
  assert_true(handledEvent.body, 'Clicking on dialog.');
  assert_true(handledEvent.dialog, 'Clicking on dialog.');
  assert_true(handledEvent.div, 'Clicking on dialog.');
}, 'Test that ancestors of modal dialog are inert.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-ancestor-is-inert.html"
}
```
