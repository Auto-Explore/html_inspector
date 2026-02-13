# html/browsers/browsing-the-web/navigating-across-documents/empty-iframe-load-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/empty-iframe-load-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>load event for empty iframe in relation to the event loop</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
setup({explicit_done:true});
let ran = false;

onload = function() {
    let iframe = document.createElement("iframe");
    iframe.onload = function() {
        test(function() {
            assert_equals(ran, false, 'Expected onload to run first');
        }, "Check execution order on load handler");
        if (ran) {
            done();
        } else {
            ran = true;
        }
    };
    document.body.appendChild(iframe);

    // Nested timeout to accommodate Gecko, because the it seems
    // the outer setTimeout takes its slot in the event queue right away
    // but the load event task takes its slot only at the end of this script.
    setTimeout(function() {
        setTimeout(function() {
            test(function() {
                assert_equals(ran, true, 'Expected nested setTimeout to run second');
            }, "Check execution order from nested timeout");
            if (ran) {
                done();
            } else {
                ran = true;
            }
        });
    });
};
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/empty-iframe-load-event.html"
}
```
