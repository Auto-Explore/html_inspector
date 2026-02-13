# html/dom/render-blocking/blocking-idl-attr.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/blocking-idl-attr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests the 'blocking' IDL attribute on link, script and style elements</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// Tests that the 'blocking' attribute follows the IDL:
// [SameObject, PutForwards=value] readonly attribute DOMTokenList blocking;

test(() => {
  const link = document.createElement('link');
  assert_true(link.blocking.supports('render'));
  assert_false(link.blocking.supports('asdf'));
}, "Supported tokens of the 'blocking' IDL attribute of the link element");

test(() => {
  const link = document.createElement('link');
  link.blocking = 'asdf';
  assert_equals(link.blocking.value, 'asdf');
}, "Setting the 'blocking' IDL attribute of the link element");

test(() => {
  const script = document.createElement('script');
  assert_true(script.blocking.supports('render'));
  assert_false(script.blocking.supports('asdf'));
}, "Supported tokens of the 'blocking' IDL attribute of the script element");

test(() => {
  const script = document.createElement('script');
  script.blocking = 'asdf';
  assert_equals(script.blocking.value, 'asdf');
}, "Setting the 'blocking' IDL attribute of the script element");

test(() => {
  const style = document.createElement('style');
  assert_true(style.blocking.supports('render'));
  assert_false(style.blocking.supports('asdf'));
}, "Supported tokens of the 'blocking' IDL attribute of the style element");

test(() => {
  const style = document.createElement('style');
  style.blocking = 'asdf';
  assert_equals(style.blocking.value, 'asdf');
}, "Setting the 'blocking' IDL attribute of the style element");
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
  "source_name": "html/dom/render-blocking/blocking-idl-attr.html"
}
```
