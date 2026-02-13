# html/semantics/popovers/popover-light-dismiss-on-scroll.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-light-dismiss-on-scroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8" />
<title>Popover should *not* light dismiss on scroll</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=help href="https://github.com/openui/open-ui/issues/240">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=scroller>
  Scroll me<br><br>
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
  ut labore et dolore magna aliqua. Enim ut sem viverra aliquet eget sit amet tellus. Massa
  sed elementum tempus egestas sed sed risus pretium. Felis bibendum ut tristique et egestas
  quis. Tortor dignissim convallis aenean et. Eu mi bibendum neque egestas congue quisque
</div>

<div popover id=popover1>
  This is popover 1
  <div popover id=popover2 anchor=anchor>
    This is popover 2
  </div>
</div>
<button onclick='popover1.showPopover();popover2.showPopover();'>Open popovers</button>

<style>
  #popover1 { top:50px; left: 50px; }
  #popover2 { top:150px; left: 50px; }
  #scroller {
    height: 150px;
    width: 150px;
    overflow-y: scroll;
    border: 1px solid black;
  }
</style>

<script>
  const popovers = document.querySelectorAll('[popover]');
  function assertAll(showing) {
    for(let popover of popovers) {
      assert_equals(popover.matches(':popover-open'),showing);
    }
  }
  async_test(t => {
    for(let popover of popovers) {
      popover.addEventListener('beforetoggle',e => {
        if (e.newState !== "closed")
          return;
        assert_unreached('Scrolling should not light-dismiss a popover');
      });
    }
    assertAll(/*showing*/false);
    popovers[0].showPopover();
    popovers[1].showPopover();
    assertAll(/*showing*/true);
    scroller.scrollTo(0, 100);
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        assertAll(/*showing*/true);
        t.done();
      });
    });
  },'Scrolling should not light-dismiss popovers');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1138,
        "byte_start": 1131,
        "col": 1,
        "line": 28
      }
    }
  ],
  "source_name": "html/semantics/popovers/popover-light-dismiss-on-scroll.html"
}
```
