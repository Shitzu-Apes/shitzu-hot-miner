<div class="body">
  <slot />
</div>

<style lang="scss">
  .body {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    min-width: 100vw;
    max-width: 100vw;
    overflow-y: auto;
    flex: 1 1 auto;

    @include breakpoint(mobile, max) {
      @include breakpoint(phone, max) {
        padding: 0.6rem 0.2rem;
      }
      padding: 1.2rem 0.3rem;
      overflow: unset;
    }
    padding: 1.8rem 0.4rem;

    :global(> *) {
      max-width: 100%;
    }
  }
</style>
