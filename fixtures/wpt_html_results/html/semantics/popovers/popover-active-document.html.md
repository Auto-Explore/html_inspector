# html/semantics/popovers/popover-active-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-active-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/10705">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  test(() => {
    const doc = document.implementation.createHTMLDocument();
    const popover = doc.createElement('div');
    popover.setAttribute('popover','');
    doc.body.appendChild(popover);
    assert_throws_dom('InvalidStateError',() => popover.showPopover());
    assert_false(popover.matches(':popover-open'));
  },'showPopover should throw when the document isn\'t active');
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
  "source_name": "html/semantics/popovers/popover-active-document.html"
}
```
