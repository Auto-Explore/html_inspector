# html/editing/the-hidden-attribute/hidden-until-found-idl.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-idl.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/#the-hidden-attribute">
<link rel=helph href="https://issues.chromium.org/issues/402108887">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=target>div</div>

<script>
test(() => {
  const target = document.getElementById('target');
  target.setAttribute('hidden', 'UNTIL-FOUND');
  assert_equals(target.hidden, 'until-found');
  target.setAttribute('hidden', 'uNtIl-FoUnD');
  assert_equals(target.hidden, 'until-found');
  target.setAttribute('hidden', 'until-found');
  assert_equals(target.hidden, 'until-found');
}, 'element.hidden should return "until-found" regardless of uppercase letters.');
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-idl.html"
}
```
