# html/semantics/popovers/hide-other-popover-side-effects.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/hide-other-popover-side-effects.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://chromium-review.googlesource.com/c/chromium/src/+/4094463/8/third_party/blink/renderer/core/html/html_element.cc#1404">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=popover1 popover=auto>popover1</div>
<div id=popover2 popover=auto>popover2</div>

<script>
test(() => {
  const popover2 = document.getElementById('popover2');
  popover1.showPopover();
  popover1.addEventListener('beforetoggle', () => {
    popover2.remove();
  });
  assert_throws_dom('InvalidStateError', () => popover2.showPopover(),
    "popover1's beforetoggle event handler removes popover2 so showPopover should throw.");
  assert_false(popover2.matches(':popover-open'), 'popover2 should not match :popover-open once it is closed.');
}, 'Removing a popover while it is opening and force closing another popover should throw an exception.');
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
  "source_name": "html/semantics/popovers/hide-other-popover-side-effects.html"
}
```
