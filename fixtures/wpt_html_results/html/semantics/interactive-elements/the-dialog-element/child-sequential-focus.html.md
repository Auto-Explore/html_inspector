# html/semantics/interactive-elements/the-dialog-element/child-sequential-focus.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/child-sequential-focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/8199">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog autofocus id=autofocusdialog data-description="dialog element with autofocus should get initial focus." class=target>
  <button tabindex="0">focusable button</button>
  <button tabindex="0" autofocus>autofocusable button</button>
</dialog>

<dialog id=keyboardfocusdialog data-description="Only keyboard-focusable elements should get dialog initial focus.">
  <button tabindex="-1">mouse focusable button</button>
  <button tabindex="0" class=target>keyboard focusable button</button>
</dialog>

<dialog id=autofocuswithoutkeyboarddialog data-description="Autofocus takes precedence over keyboard-focusable requirement.">
  <button tabindex="0">keyboard focusable button</button>
  <button tabindex="-1" autofocus class=target>mouse focusable autofocus button</button>
</dialog>

<dialog id=subtree data-description="Only keyboard-focusable elements should get dialog initial focus including in subtrees.">
  <div>
    <button tabindex="-1">mouse focusable button</button>
    <button tabindex="0" class=target>keyboard focusable button</button>
  </div>
</dialog>

<dialog id=nestedbuttons data-description="Only keyboard-focusable elements should get dialog initial focus including in nested buttons.">
  <button tabindex="-1">
    <span>mouse focusable button</span>
    <button tabindex="-1">nested mouse focusable button</button>
  </button>
  <button tabindex="0" class=target>keyboard focusable button</button>
</dialog>

<script>
document.querySelectorAll('dialog').forEach(dialog => {
  test(t => {
    let target = dialog.querySelector('.target');
    if (dialog.classList.contains('target')) {
      target = dialog;
    }
    t.add_cleanup(() => {
      if (dialog.open)
        dialog.close();
    });

    dialog.showModal();
    assert_equals(document.activeElement, target,
      'showModal: the target element did not receive initial focus.');
    dialog.close();

    dialog.show();
    assert_equals(document.activeElement, target,
      'show: the target element did not receive initial focus.');
    dialog.close();
  }, dialog.dataset.description);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 447,
        "byte_start": 416,
        "col": 3,
        "line": 9
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/child-sequential-focus.html"
}
```
