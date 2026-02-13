# html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/in-order.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/in-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/C/#list-of-scripts-that-will-execute-in-order-as-soon-as-possible">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="helper.js"></script>
<body>
<script>
const t = async_test('Script elements (in-order) still block subsequent in-order scripts in the original Document after moved to another Document');
const start_time = performance.now();
const iframe = document.createElement('iframe');
document.body.appendChild(iframe);

const onScript2Evaluated = t.step_func_done(() => {
  // `script1` should remain the
  // #list-of-scripts-that-will-execute-in-order-as-soon-as-possible of the
  // original Document and thus blocks `script2` evaluation until it is loaded.
  assert_greater_than(performance.now() - start_time, 2000,
      'In-order scripts should block subsequent in-order scripts');
});

const script1 = document.createElement('script');
script1.async = false;
script1.setAttribute('src', '../../resources/throw.js?pipe=trickle(d2)');
document.body.appendChild(script1);

const script2 = document.createElement('script');
script2.async = false;
script2.setAttribute('src', 'data:text/javascript,onScript2Evaluated()');
document.body.appendChild(script2);

t.step_timeout(() => {
  iframe.contentDocument.body.appendChild(script1);
}, 1000);
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
  "source_name": "html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/in-order.html"
}
```
