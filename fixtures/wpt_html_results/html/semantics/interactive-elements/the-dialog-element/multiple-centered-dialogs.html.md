# html/semantics/interactive-elements/the-dialog-element/multiple-centered-dialogs.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/multiple-centered-dialogs.html",
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
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
body {
  height: 10000px;
}

dialog {
  padding: 0;
  height: 50px;
  width: 50px;
}

#console {
  position: fixed;
}
</style>

<dialog id="top-dialog"></dialog>
<dialog id="first-middle-dialog"></dialog>
<dialog id="second-middle-dialog" style="left: 100px"></dialog>
<dialog id="bottom-dialog"></dialog>

<script>
test(() => {
  function documentHeight() {
    // clientHeight is an integer, but we want the correct floating point
    // value.  Start a binary search at clientHeight-1 and clientHeight+1.
    let min = document.documentElement.clientHeight;
    let max = min + 1;
    --min;

    // binary search with media queries to find the correct height
    for (let iter = 0; iter < 10; ++iter) {
      let test = (min + max) / 2;
      if (window.matchMedia(`(min-height: ${test}px)`).matches)
        min = test;
      else
        max = test;
    }
    return min;
  }
  function expectedTop(dialog) {
    let height = documentHeight();
    return (height - dialog.getBoundingClientRect().height) / 2;
  }

  function showAndTest(id) {
    dialog = document.getElementById(id);
    dialog.showModal();
    assert_approx_equals(dialog.getBoundingClientRect().top, expectedTop(dialog), 0.05, id);
  }

  showAndTest('top-dialog');

  window.scroll(0, 100);
  showAndTest('first-middle-dialog');
  showAndTest('second-middle-dialog');

  window.scroll(0, 200);
  showAndTest('bottom-dialog');
}, 'Test that multiple dialogs are centered properly.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/multiple-centered-dialogs.html"
}
```
