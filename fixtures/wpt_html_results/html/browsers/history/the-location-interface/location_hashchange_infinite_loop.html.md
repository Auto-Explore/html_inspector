# html/browsers/history/the-location-interface/location_hashchange_infinite_loop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location_hashchange_infinite_loop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Set window.location.hash to same value while handling event hashchange</title>
<link rel="author" href="mailto:cristianb@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-location-hash">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
let count = 0;

window.onhashchange = () => {
    count += 1;
    window.location.hash = 'running';
}

promise_test(async t => {
    window.location.hash = 'running';

    await t.step_wait(() => count === 1);
}, 'Setting window.location.hash to same value while handling hashchange event shouldn\'t cause an infinite loop.');
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
  "source_name": "html/browsers/history/the-location-interface/location_hashchange_infinite_loop.html"
}
```
