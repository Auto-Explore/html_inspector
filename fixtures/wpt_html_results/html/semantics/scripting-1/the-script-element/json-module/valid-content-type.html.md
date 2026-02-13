# html/semantics/scripting-1/the-script-element/json-module/valid-content-type.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/valid-content-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>JSON modules: Content-Type</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
function check(t, v) {
  t.step(() => {
    assert_equals(typeof v, "object");
    assert_array_equals(Object.keys(v), ["test"]);
    assert_equals(v.test, true);
    t.done();
  });
}
const t1 = async_test("text/json");
const t2 = async_test("application/json");
const t3 = async_test("text/html+json");
const t4 = async_test("image/svg+json");
const t5 = async_test("text/json;boundary=something");
const t6 = async_test("text/json;foo=bar");
const t7 = async_test("text/json;+json");
const t8 = async_test("text/html+xml+json");
</script>
<script type="module" onerror="t1.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/json" with { type: "json"};
check(t1, v);
</script>
<script type="module" onerror="t2.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=application/json" with { type: "json"};
check(t2, v);
</script>
<script type="module" onerror="t3.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/html%2Bjson" with { type: "json"};
check(t3, v);
</script>
<script type="module" onerror="t4.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=image/svg%2Bjson" with { type: "json"};
check(t4, v);
</script>
<script type="module" onerror="t5.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/json;boundary=something" with { type: "json"};
check(t5, v);
</script>
<script type="module" onerror="t6.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/json;foo=bar" with { type: "json"};
check(t6, v);
</script>
<script type="module" onerror="t7.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/json;%2Bjson" with { type: "json"};
check(t7, v);
</script>
<script type="module" onerror="t8.step(() => assert_unreached(event))">
import v from "../serve-with-content-type.py?fn=json-module/module.json&ct=text/html%2Bxml%2Bjson" with { type: "json"};
check(t8, v);
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/valid-content-type.html"
}
```
