# html/semantics/scripting-1/the-script-element/module/type.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Type attribute of module scripts</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
window.t1 = async_test('type="module"');
window.t2 = async_test('type="MODULE"');
window.t3 = async_test('type="Module"');
window.t4 = async_test('type="module "');
window.t5 = async_test('type=" module"');
</script>
<script type="module">window.t1.done();</script>
<script type="MODULE">window.t2.done();</script>
<script type="Module">window.t3.done();</script>
<script type="module ">window.t4.unreached_func('Unexpectedly evaluated');</script>
<script type=" module">window.t5.unreached_func('Unexpectedly evaluated');</script>
<script type="module">
window.t1.unreached_func('Unexpectedly not evaluated')();
window.t2.unreached_func('Unexpectedly not evaluated')();
window.t3.unreached_func('Unexpectedly not evaluated')();
window.t4.done();
window.t5.done();
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/type.html"
}
```
