# html/semantics/popovers/popover-document-open.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-document-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div popover id=popover1>Popover</div>

<script>
  window.onload = () => {
    test((t) => {
      const popover1 = document.querySelector('#popover1');
      popover1.showPopover();
      assert_true(popover1.matches(':popover-open'));
      assert_true(!document.querySelector('#popover2'));
      document.open();
      document.write('<!DOCTYPE html><div popover id=popover2>Popover</div>');
      document.close();
      assert_true(!document.querySelector('#popover1'),'popover1 should be removed from the document');
      assert_true(!!document.querySelector('#popover2'),'popover2 should be in the document');
      assert_false(popover1.matches(':popover-open'),'popover1 should have been hidden when it was removed from the document');
      assert_false(popover1.matches(':popover-open'),'popover2 shouldn\'t be showing yet');
      popover2.showPopover();
      assert_true(popover2.matches(':popover-open'),'popover2 should be able to be shown');
      popover2.hidePopover();
    },'document.open should not break popovers');
  };
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
  "source_name": "html/semantics/popovers/popover-document-open.html"
}
```
