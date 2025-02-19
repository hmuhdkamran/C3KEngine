type Subscriber<T = any> = (payload: T) => void;

export class PubSub {
    private static subscribers: Map<string, Subscriber[]> = new Map();

    /**
     * Subscribes to an event.
     * @param event - The name of the event.
     * @param subscriber - The callback function to invoke when the event is published.
     */
    public static subscribe<T = any>(event: string, subscriber: Subscriber<T>): void {
        if (!this.subscribers.has(event)) {
            this.subscribers.set(event, []);
        }
        this.subscribers.get(event)!.push(subscriber);
    }

    /**
     * Unsubscribes from an event.
     * @param event - The name of the event.
     * @param subscriber - The callback function to remove from the subscription list.
     */
    public static unsubscribe<T = any>(event: string, subscriber: Subscriber<T>): void {
        if (this.subscribers.has(event)) {
            const updatedSubscribers = this.subscribers.get(event)!.filter(sub => sub !== subscriber);
            if (updatedSubscribers.length > 0) {
                this.subscribers.set(event, updatedSubscribers);
            } else {
                this.subscribers.delete(event);
            }
        }
    }

    /**
     * Publishes an event, invoking all subscribed callbacks with the provided payload.
     * @param event - The name of the event.
     * @param payload - The data to pass to each subscriber's callback.
     */
    public static publish<T = any>(event: string, payload: T): void {
        if (this.subscribers.has(event)) {
            this.subscribers.get(event)!.forEach(subscriber => {
                subscriber(payload);
            });
        }
    }
}
