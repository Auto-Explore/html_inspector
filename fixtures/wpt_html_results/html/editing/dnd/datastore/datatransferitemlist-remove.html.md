# html/editing/dnd/datastore/datatransferitemlist-remove.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/datastore/datatransferitemlist-remove.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>DataTransferItemList remove() method</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

// https://github.com/whatwg/html/issues/2925
test(() => {
  const dt = new DataTransfer();

  // Must not throw
  dt.items.remove(0);
  dt.items.remove(1);

  dt.items.add("data", "text/plain");

  // Must not throw
  dt.items.remove(1);
}, "remove()ing an out-of-bounds index does nothing");

test(() => {
  const file = new File(["🕺💃"], "test.png", {
        type: "image/png"
  });

  const dt = new DataTransfer();
  dt.items.add(file);

  let item = dt.items[0];
  dt.items.remove(0);

  assert_equals(item.kind, "");
  assert_equals(item.type, "");
  assert_equals(item.getAsFile(), null);
}, "remove()ing an item will put the associated DataTransferItem object in the disabled mode");
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
  "source_name": "html/editing/dnd/datastore/datatransferitemlist-remove.html"
}
```
