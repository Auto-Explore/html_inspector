# html/semantics/popovers/popover-shadowhost-focus.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-shadowhost-focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/8994">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div popover=auto tabindex=0 data-test="autofocus=true, delegatesfocus=false" autofocus class=should-be-focused>
  <template shadowrootmode=open>
    <button autofocus>autofocus button</button>
  </template>
</div>

<!-- The autofocus popover is what focus() gets called on, but since it has a
  delegatesFocus shadowroot, focus() itself goes into the shadowroot. -->
<div popover=auto tabindex=0 data-test="autofocus=true, delegatesfocus=true" autofocus>
  <template shadowrootmode=open shadowrootdelegatesfocus>
    <button autofocus class=should-be-focused>autofocus button</button>
  </template>
</div>

<div popover=auto tabindex=0 data-test="autofocus=false, delegatesfocus=false">
  <template shadowrootmode=open>
    <button autofocus>autofocus button</button>
  </template>
</div>

<div popover=auto tabindex=0 data-test="autofocus=false, delegatesfocus=true">
  <template shadowrootmode=open shadowrootdelegatesfocus>
    <button autofocus>autofocus button</button>
  </template>
</div>

<script>
document.querySelectorAll('body > [popover]').forEach(popover => {
  promise_test(async () => {
    const expectedFocusedElement = (popover.matches('.should-be-focused') ? popover : null)
      || popover.querySelector('.should-be-focused')
      || popover.shadowRoot.querySelector('.should-be-focused')
      || document.body;

    popover.showPopover();

    let actualFocusedElement = document.activeElement;
    if (actualFocusedElement.shadowRoot && actualFocusedElement.shadowRoot.activeElement) {
      actualFocusedElement = actualFocusedElement.shadowRoot.activeElement;
    }

    popover.hidePopover();

    // Resetting focus may happen asynchronously
    await new Promise(resolve => requestAnimationFrame(resolve));

    assert_equals(actualFocusedElement, expectedFocusedElement);
  }, popover.getAttribute('data-test'));
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
        "byte_end": 409,
        "byte_start": 391,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.autofocus.multiple_in_scoping_root",
      "message": "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 801,
        "byte_start": 759,
        "col": 5,
        "line": 17
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
  "source_name": "html/semantics/popovers/popover-shadowhost-focus.html"
}
```
