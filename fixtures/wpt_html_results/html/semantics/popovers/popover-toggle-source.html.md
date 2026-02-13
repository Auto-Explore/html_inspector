# html/semantics/popovers/popover-toggle-source.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-toggle-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9111">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/toggle-event-source-test.js"></script>

<button id=popoversource popovertarget=popover>popovertarget source</button>
<button id=commandsource commandfor=popover command=show-popover>command source</button>
<div id=popover popover=auto>
  popover
  <button id=popoversourcehide popovertarget=popover>popovertarget source</button>
  <button id=commandsourcehide commandfor=popover command=hide-popover>command source</button>
</div>

<script>
const popoversource = document.getElementById('popoversource');
const popoversourcehide = document.getElementById('popoversourcehide');
const commandsource = document.getElementById('commandsource');
const commandsourcehide = document.getElementById('commandsourcehide');
const popover = document.getElementById('popover');

let beforetoggleEvent = null;
let toggleEvent = null;
popover.addEventListener('beforetoggle', event => beforetoggleEvent = event);
popover.addEventListener('toggle', event => toggleEvent = event);

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: showPopover() without source.',
  target: popover,
  openFunc: async () => popover.showPopover(),
  closeFunc: async () => popover.hidePopover(),
  openSource: null,
  closeSource: null
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: showPopover() with source.',
  target: popover,
  openFunc: async () => popover.showPopover({source: popoversource}),
  closeFunc: async () => popover.hidePopover(),
  openSource: popoversource,
  closeSource: null
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: Calling click() on a popovertarget button.',
  target: popover,
  openFunc: async () => popoversource.click(),
  closeFunc: async () => popoversourcehide.click(),
  openSource: popoversource,
  closeSource: popoversourcehide
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: Calling click() on a command button.',
  target: popover,
  openFunc: async () => commandsource.click(),
  closeFunc: async () => commandsourcehide.click(),
  openSource: commandsource,
  closeSource: commandsourcehide
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: showPopover() then popovertarget button.',
  target: popover,
  openFunc: async () => popover.showPopover(),
  closeFunc: async () => popoversourcehide.click(),
  openSource: null,
  closeSource: popoversourcehide
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: showPopover(invoker) then popovertarget button.',
  target: popover,
  openFunc: async () => popover.showPopover({source: popoversource}),
  closeFunc: async () => popoversourcehide.click(),
  openSource: popoversource,
  closeSource: popoversourcehide
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on popover elements: popovertarget button then hidePopover().',
  target: popover,
  openFunc: async () => popoversource.click(),
  closeFunc: async () => popover.hidePopover(),
  openSource: popoversource,
  closeSource: null
});
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
  "source_name": "html/semantics/popovers/popover-toggle-source.html"
}
```
