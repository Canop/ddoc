document.addEventListener('DOMContentLoaded', () => {
    const strip = document.querySelector('.image-strip');
    if (!strip) return;
    const bubble = document.createElement('div');
    bubble.className = 'image-bubble';
    const bubbleImg = document.createElement('img');
    bubble.appendChild(bubbleImg);
    strip.appendChild(bubble);

    strip.querySelectorAll('img').forEach(thumb => {
        thumb.addEventListener('mouseenter', () => {
            bubbleImg.src = thumb.src;

            const stripRect = strip.getBoundingClientRect();
            const thumbRect = thumb.getBoundingClientRect();
            const arrowX = thumbRect.left - stripRect.left + thumbRect.width / 2;
            bubble.style.setProperty('--arrow-x', arrowX + 'px');

            const above = stripRect.top > window.innerHeight - stripRect.bottom;
            bubble.className = 'image-bubble ' + (above ? 'above' : 'below');
            bubble.style.display = 'block';
        });
        thumb.addEventListener('mouseleave', () => {
            bubble.style.display = 'none';
        });
    });
});
