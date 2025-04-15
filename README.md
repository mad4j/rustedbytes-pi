# rustedbytes-pi
Fast computation of PI digits in Rust

Per calcolare Pi (π) utilizzando l'algoritmo di Chudnovsky, è necessario implementare una formula che utilizza serie ipergeometriche. L'algoritmo di Chudnovsky è molto efficiente e converge rapidamente, permettendo di calcolare Pi a un'elevata precisione. La formula richiede calcoli di potenza, fattoriali e radici quadrate.
Formula di Chudnovsky:
Code

1/π = 12 * Σ(k=0 to ∞) [ (6k)! * (545140134k + 13591409) ] / [(3k)! * (k!)^3 * 640320^k ]
Come calcolare Pi passo dopo passo:
1. Definire i parametri:
k: un indice che va da 0 a un valore sufficientemente grande per ottenere la precisione desiderata (es. 100, 1000, ecc.).
n: il numero di cifre di Pi che si desidera calcolare.
2. Calcolare il termine della serie:
Per ogni valore di k, calcolare il termine della serie utilizzando la formula sopra, con i seguenti passaggi:
Calcolare (6k)! (fattoriale di 6k).
Calcolare (3k)! (fattoriale di 3k).
Calcolare (k!)^3 (il cubo del fattoriale di k).
Calcolare 640320^k (640320 elevato a k).
Calcolare 545140134k + 13591409.
Calcolare il rapporto tra i due termini.
3. Sommare i termini:
Sommare tutti i termini della serie ottenuti al passo precedente.
4. Calcolare Pi:
Moltiplicare il risultato della somma per 12.
Calcolare il reciproco del risultato (1 diviso il risultato).
Implementazione in un linguaggio di programmazione:
L'implementazione richiede un linguaggio di programmazione con supporto per calcoli di precisione elevata (es. Python con la libreria decimal o altri linguaggi con supporto per numeri a doppia precisione). Il codice deve gestire i calcoli fattoriali e le somme di serie in modo efficiente per ottenere una precisione desiderata.
Esempio in Python:
Python

from decimal import Decimal, getcontext

def pi_chudnovsky(num_digits):
    getcontext().prec = num_digits + 10  # set precision
    pi = Decimal(0)
    k = Decimal(0)
    while len(str(pi)) < num_digits:
        term = (Decimal(6).factorial(6 * k) * (Decimal(545140134) * k + Decimal(13591409))) / (Decimal(3).factorial(3 * k) * Decimal(k).factorial(k) ** 3 * Decimal(640320) ** k)
        pi += term
        k += 1
    return Decimal(1) / (12 * pi)

print(pi_chudnovsky(100))
Considerazioni:
L'algoritmo di Chudnovsky è molto efficiente per il calcolo di Pi, ma richiede un'implementazione corretta in un linguaggio di programmazione e la gestione della precisione per ottenere risultati accurati.
La velocità di calcolo dipende dalla precisione desiderata e dalla potenza di elaborazione del dispositivo.
Esistono implementazioni ottimizzate dell'algoritmo di Chudnovsky disponibili online, che possono essere utilizzate come riferimento.
