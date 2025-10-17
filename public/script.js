// Slide Navigation System
let currentSlide = 0;
const totalSlides = 9;

// Initialize
document.addEventListener('DOMContentLoaded', function() {
    setupNavigation();
    updateSlide();
    setupKeyboardNavigation();
    setupDotNavigation();
    setupSwipeGestures();
});

// Navigation Setup
function setupNavigation() {
    // Button navigation
    document.querySelector('.nav-btn.prev').addEventListener('click', () => changeSlide(-1));
    document.querySelector('.nav-btn.next').addEventListener('click', () => changeSlide(1));
}

// Change Slide Function
function changeSlide(direction) {
    const newSlide = currentSlide + direction;
    
    if (newSlide >= 0 && newSlide < totalSlides) {
        currentSlide = newSlide;
        updateSlide();
    }
}

// Go to specific slide
function goToSlide(slideNumber) {
    if (slideNumber >= 0 && slideNumber < totalSlides) {
        currentSlide = slideNumber;
        updateSlide();
    }
}

// Update slide display
function updateSlide() {
    // Update slides
    document.querySelectorAll('.slide').forEach((slide, index) => {
        if (index === currentSlide) {
            slide.classList.add('active');
        } else {
            slide.classList.remove('active');
        }
    });
    
    // Update dots
    document.querySelectorAll('.dot').forEach((dot, index) => {
        if (index === currentSlide) {
            dot.classList.add('active');
        } else {
            dot.classList.remove('active');
        }
    });
    
    // Update progress bar
    const progressBar = document.querySelector('.progress-bar');
    const progressWidth = ((currentSlide + 1) / totalSlides) * 100;
    progressBar.style.width = progressWidth + '%';
    
    // Update navigation buttons
    const prevBtn = document.querySelector('.nav-btn.prev');
    const nextBtn = document.querySelector('.nav-btn.next');
    
    prevBtn.style.opacity = currentSlide === 0 ? '0.3' : '1';
    prevBtn.style.cursor = currentSlide === 0 ? 'not-allowed' : 'pointer';
    
    nextBtn.style.opacity = currentSlide === totalSlides - 1 ? '0.3' : '1';
    nextBtn.style.cursor = currentSlide === totalSlides - 1 ? 'not-allowed' : 'pointer';
}

// Keyboard Navigation
function setupKeyboardNavigation() {
    document.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
            changeSlide(-1);
        } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown' || e.key === ' ') {
            e.preventDefault();
            changeSlide(1);
        } else if (e.key === 'Home') {
            goToSlide(0);
        } else if (e.key === 'End') {
            goToSlide(totalSlides - 1);
        } else if (e.key >= '1' && e.key <= '9') {
            goToSlide(parseInt(e.key) - 1);
        }
    });
}

// Dot Navigation
function setupDotNavigation() {
    document.querySelectorAll('.dot').forEach((dot, index) => {
        dot.addEventListener('click', () => {
            goToSlide(index);
        });
    });
}

// Touch/Swipe Support
function setupSwipeGestures() {
    let touchStartX = 0;
    let touchEndX = 0;
    let touchStartY = 0;
    let touchEndY = 0;
    
    document.addEventListener('touchstart', (e) => {
        touchStartX = e.changedTouches[0].screenX;
        touchStartY = e.changedTouches[0].screenY;
    });
    
    document.addEventListener('touchend', (e) => {
        touchEndX = e.changedTouches[0].screenX;
        touchEndY = e.changedTouches[0].screenY;
        handleSwipe();
    });
    
    function handleSwipe() {
        const swipeThreshold = 50;
        const diffX = touchEndX - touchStartX;
        const diffY = touchEndY - touchStartY;
        
        // Check if horizontal swipe is more prominent
        if (Math.abs(diffX) > Math.abs(diffY)) {
            if (diffX > swipeThreshold) {
                changeSlide(-1); // Swipe right - go to previous
            } else if (diffX < -swipeThreshold) {
                changeSlide(1); // Swipe left - go to next
            }
        } else {
            // Vertical swipe
            if (diffY > swipeThreshold) {
                changeSlide(-1); // Swipe down - go to previous
            } else if (diffY < -swipeThreshold) {
                changeSlide(1); // Swipe up - go to next
            }
        }
    }
}

// Mouse Wheel Navigation
document.addEventListener('wheel', (e) => {
    // Debounce wheel events
    if (e.deltaY > 50) {
        changeSlide(1);
    } else if (e.deltaY < -50) {
        changeSlide(-1);
    }
}, { passive: true });

// Visibility API - Pause animations when tab is not visible
document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
        // Pause any animations if needed
    } else {
        // Resume animations if needed
    }
});

// Preload images for smooth transitions
window.addEventListener('load', () => {
    const images = [];
    for (let i = 0; i <= 116; i++) {
        const img = new Image();
        img.src = `images/cowboy-pdf-${String(i).padStart(3, '0')}.png`;
    }
});

// Add smooth parallax effect to background
let ticking = false;
function updateParallax() {
    const slides = document.querySelectorAll('.slide.active .slide-bg');
    slides.forEach(bg => {
        const scrolled = currentSlide * 100;
        bg.style.transform = `translateY(${scrolled * 0.1}px) scale(1.1)`;
    });
}

// Request animation frame for smooth animations
function requestTick() {
    if (!ticking) {
        window.requestAnimationFrame(updateParallax);
        ticking = true;
    }
}

// Add page visibility handling for better performance
let slideTimer;
function startAutoAdvance() {
    slideTimer = setInterval(() => {
        if (currentSlide < totalSlides - 1) {
            changeSlide(1);
        }
    }, 10000); // Auto-advance every 10 seconds
}

function stopAutoAdvance() {
    if (slideTimer) {
        clearInterval(slideTimer);
    }
}

// Optional: Enable auto-advance (commented out by default)
// startAutoAdvance();

// Stop auto-advance on user interaction
document.addEventListener('click', stopAutoAdvance);
document.addEventListener('keydown', stopAutoAdvance);
document.addEventListener('touchstart', stopAutoAdvance);