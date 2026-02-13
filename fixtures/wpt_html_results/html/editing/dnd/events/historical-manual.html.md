# html/editing/dnd/events/historical-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/historical-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Historical drag-and-drop features</title>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
.test-square {
  width: 100px;
  height: 100px;
}

#draggable {
  background: orange;
}

#dropzone {
  background: blue;
}
</style>

<p>Drag the orange square onto the blue square and release it.</p>

<div draggable="true" id="draggable" class="test-square" ondragstart="event.dataTransfer.setData('text/plain', null)"></div>
<div id="dropzone" class="test-square"></div>

<script>
"use strict";

async_test(t => {
  let dragexitCount = 0;
  document.addEventListener("dragexit", () => {
    ++dragexitCount;
  });

  // Prevent the event to allow drop
  document.addEventListener("dragover", e => {
    e.preventDefault();
  });

  document.addEventListener("drop", t.step_func_done(() => {
    assert_equals(dragexitCount, 0);
  }));
}, `dragexit must not fire during drag-and-drop`);
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
  "source_name": "html/editing/dnd/events/historical-manual.html"
}
```
